<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Application, Graphics, Text, TextStyle, Container } from 'pixi.js';
  import { forceSimulation, forceLink, forceManyBody, forceCenter, forceCollide } from 'd3-force';
  import type { Simulation, SimulationNodeDatum, SimulationLinkDatum } from 'd3-force';
  import type { GraphData } from '$lib/api/graphApi';

  let {
    data,
    onCategoryTap,
    onItemToggle,
  }: {
    data: GraphData;
    onCategoryTap: (categoryId: number) => void;
    onItemToggle: (itemId: number, done: boolean) => Promise<void>;
  } = $props();

  let canvasContainer: HTMLDivElement;
  let app: Application | null = null;
  let simulation: Simulation<SimNode, SimLink> | null = null;
  let animFrameId: number | null = null;
  let destroyed = false;

  interface SimNode extends SimulationNodeDatum {
    id: string;
    rawId: number;
    nodeType: 'item' | 'tag' | 'category';
    label: string;
    categoryId: number | null;
    done: boolean | null;
    radius: number;
    graphics: Graphics | null;
    textObj: Text | null;
  }

  interface SimLink extends SimulationLinkDatum<SimNode> {
    source: SimNode;
    target: SimNode;
  }

  function getCssVar(name: string): string {
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  }

  function hexToNum(hex: string): number {
    return parseInt(hex.replace('#', ''), 16);
  }

  function truncateLabel(label: string, maxLen: number): string {
    if (label.length <= maxLen) return label;
    return label.slice(0, maxLen - 1) + '…';
  }

  // Tap detection threshold (pixels)
  const TAP_THRESHOLD = 6;

  onMount(async () => {
    if (destroyed) return;

    const width = canvasContainer.clientWidth;
    const height = canvasContainer.clientHeight;

    // Read theme colors
    const paperColor = getCssVar('--color-paper') || '#f8f7f3';
    const skyColor = getCssVar('--color-accent-sky-strong') || '#8ea9cf';
    const skyStrongColor = getCssVar('--color-accent-sky') || '#a8bddb';
    const mintColor = getCssVar('--color-accent-mint') || '#bfd9c8';
    const peachColor = getCssVar('--color-accent-peach') || '#e9c1ad';
    const inkColor = getCssVar('--color-ink') || '#5b5852';
    const inkMutedColor = getCssVar('--color-ink-muted') || '#7a776f';
    const canvasColor = getCssVar('--color-canvas') || '#f2efe8';

    // Create PixiJS application (force WebGL for iOS WKWebView compatibility)
    app = new Application();
    await app.init({
      preference: 'webgl',
      width,
      height,
      backgroundColor: hexToNum(paperColor),
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
    });

    if (destroyed) {
      app.destroy();
      app = null;
      return;
    }

    canvasContainer.appendChild(app.canvas as HTMLCanvasElement);

    // Create a world container for pan/zoom
    const world = new Container();
    app.stage.addChild(world);

    const edgeGraphics = new Graphics();
    world.addChild(edgeGraphics);

    // Highlight edge layer (drawn on top of normal edges)
    const highlightEdgeGraphics = new Graphics();
    world.addChild(highlightEdgeGraphics);

    // Tag highlight state
    let highlightedTagId: string | null = null;

    // Build simulation nodes
    const radiusMap = { category: 12, tag: 10, item: 8 };
    const simNodes: SimNode[] = data.nodes.map((n) => ({
      id: `${n.node_type}-${n.id}`,
      rawId: n.id,
      nodeType: n.node_type,
      label: n.label,
      categoryId: n.category_id,
      done: n.done,
      radius: radiusMap[n.node_type] ?? 9,
      graphics: null,
      textObj: null,
      x: width / 2 + (Math.random() - 0.5) * 80,
      y: height / 2 + (Math.random() - 0.5) * 80,
    }));

    const nodeMap = new Map<string, SimNode>();
    for (const n of simNodes) {
      nodeMap.set(n.id, n);
    }

    // Build simulation links from string-based source/target
    const simLinks: SimLink[] = [];
    for (const edge of data.edges) {
      const source = nodeMap.get(edge.source);
      const target = nodeMap.get(edge.target);
      if (source && target) {
        simLinks.push({ source, target });
      }
    }

    // Helper: redraw a single node graphic
    function redrawNode(node: SimNode) {
      if (!node.graphics) return;
      const g = node.graphics;
      g.clear();
      const borderColor = hexToNum(inkMutedColor);

      if (node.nodeType === 'category') {
        g.circle(0, 0, node.radius);
        g.fill({ color: hexToNum(mintColor) });
        g.stroke({ color: borderColor, width: 1.5, alpha: 0.4 });
      } else if (node.nodeType === 'tag') {
        g.circle(0, 0, node.radius);
        g.fill({ color: hexToNum(skyColor) });
        g.stroke({ color: borderColor, width: 1.5, alpha: 0.4 });
      } else {
        const color = node.done ? hexToNum(peachColor) : hexToNum(canvasColor);
        g.circle(0, 0, node.radius);
        g.fill({ color });
        g.stroke({ color: borderColor, width: 1.5, alpha: 0.4 });
      }
    }

    // Create PixiJS objects for each node
    const categoryTextStyle = new TextStyle({
      fontSize: 12,
      fontWeight: 'bold',
      fill: hexToNum(inkColor),
    });
    const tagTextStyle = new TextStyle({
      fontSize: 11,
      fontWeight: 'bold',
      fill: hexToNum(inkColor),
    });
    const itemTextStyle = new TextStyle({
      fontSize: 10,
      fill: hexToNum(inkMutedColor),
    });

    for (const node of simNodes) {
      const g = new Graphics();
      node.graphics = g;
      redrawNode(node);

      g.eventMode = 'static';
      g.cursor = 'pointer';
      world.addChild(g);

      let labelText: string;
      let labelStyle: TextStyle;
      if (node.nodeType === 'category') {
        labelText = truncateLabel(node.label, 12);
        labelStyle = categoryTextStyle;
      } else if (node.nodeType === 'tag') {
        labelText = `#${truncateLabel(node.label, 11)}`;
        labelStyle = tagTextStyle;
      } else {
        labelText = truncateLabel(node.label, 10);
        labelStyle = itemTextStyle;
      }

      const text = new Text({ text: labelText, style: labelStyle });
      text.anchor.set(0.5, 0);
      world.addChild(text);
      node.textObj = text;

      // Drag + tap detection
      let dragging = false;
      let didMove = false;
      let dragOffset = { x: 0, y: 0 };
      let downPos = { x: 0, y: 0 };

      g.on('pointerdown', (e) => {
        e.stopPropagation();
        dragging = true;
        didMove = false;
        const worldPos = world.toLocal(e.global);
        dragOffset.x = worldPos.x - (node.x ?? 0);
        dragOffset.y = worldPos.y - (node.y ?? 0);
        downPos = { x: e.global.x, y: e.global.y };
        node.fx = node.x;
        node.fy = node.y;
        simulation?.alphaTarget(0.08).restart();
        // Highlight on press for tag nodes
        if (node.nodeType === 'tag') {
          highlightedTagId = node.id;
        }
      });

      const onMove = (e: any) => {
        if (!dragging) return;
        const dx = e.global.x - downPos.x;
        const dy = e.global.y - downPos.y;
        if (Math.abs(dx) > TAP_THRESHOLD || Math.abs(dy) > TAP_THRESHOLD) {
          didMove = true;
        }
        const worldPos = world.toLocal(e.global);
        node.fx = worldPos.x - dragOffset.x;
        node.fy = worldPos.y - dragOffset.y;
      };

      const onUp = async () => {
        if (!dragging) return;
        dragging = false;
        node.fx = null;
        node.fy = null;
        simulation?.alphaTarget(0);

        // Release tag highlight
        if (node.nodeType === 'tag') {
          highlightedTagId = null;
        }

        // Tap (not drag)
        if (!didMove) {
          if (node.nodeType === 'category') {
            onCategoryTap(node.rawId);
          } else if (node.nodeType === 'item') {
            const newDone = !node.done;
            try {
              await onItemToggle(node.rawId, newDone);
              node.done = newDone;
              redrawNode(node);
            } catch (err) {
              console.error('Failed to toggle item:', err);
            }
          }
        }
      };

      g.on('globalpointermove', onMove);
      g.on('pointerup', onUp);
      g.on('pointerupoutside', onUp);
    }

    // Setup d3-force simulation
    simulation = forceSimulation(simNodes)
      .force(
        'link',
        forceLink<SimNode, SimLink>(simLinks)
          .id((d) => d.id)
          .distance(55)
          .strength(0.8)
      )
      .force('charge', forceManyBody().strength(-60))
      .force('center', forceCenter(width / 2, height / 2).strength(0.15))
      .force(
        'collide',
        forceCollide<SimNode>().radius((d) => d.radius + 4)
      )
      .alphaDecay(0.05);

    // Render loop
    function render() {
      if (destroyed || !app) return;

      // Draw normal edges
      edgeGraphics.clear();
      highlightEdgeGraphics.clear();

      const normalPaths: { sx: number; sy: number; tx: number; ty: number }[] = [];
      const highlightPaths: { sx: number; sy: number; tx: number; ty: number }[] = [];

      for (const link of simLinks) {
        const sx = link.source.x ?? 0;
        const sy = link.source.y ?? 0;
        const tx = link.target.x ?? 0;
        const ty = link.target.y ?? 0;
        const path = { sx, sy, tx, ty };

        if (highlightedTagId && (link.source.id === highlightedTagId || link.target.id === highlightedTagId)) {
          highlightPaths.push(path);
        } else {
          normalPaths.push(path);
        }
      }

      // Draw normal edges (dimmed if highlight active)
      for (const p of normalPaths) {
        edgeGraphics.moveTo(p.sx, p.sy);
        edgeGraphics.lineTo(p.tx, p.ty);
      }
      edgeGraphics.stroke({
        color: hexToNum(inkMutedColor),
        width: 1.5,
        alpha: highlightedTagId ? 0.12 : 0.4,
      });

      // Draw highlighted edges
      if (highlightPaths.length > 0) {
        for (const p of highlightPaths) {
          highlightEdgeGraphics.moveTo(p.sx, p.sy);
          highlightEdgeGraphics.lineTo(p.tx, p.ty);
        }
        highlightEdgeGraphics.stroke({
          color: hexToNum(skyStrongColor),
          width: 2.5,
          alpha: 0.8,
        });
      }

      // Update node positions
      for (const node of simNodes) {
        if (node.graphics) {
          node.graphics.x = node.x ?? 0;
          node.graphics.y = node.y ?? 0;
          // Dim non-connected nodes when highlight active
          if (highlightedTagId) {
            const isConnected = node.id === highlightedTagId ||
              simLinks.some(l =>
                (l.source.id === highlightedTagId && l.target.id === node.id) ||
                (l.target.id === highlightedTagId && l.source.id === node.id)
              );
            node.graphics.alpha = isConnected ? 1 : 0.3;
            if (node.textObj) node.textObj.alpha = isConnected ? 1 : 0.3;
          } else {
            node.graphics.alpha = 1;
            if (node.textObj) node.textObj.alpha = 1;
          }
        }
        if (node.textObj) {
          node.textObj.x = node.x ?? 0;
          node.textObj.y = (node.y ?? 0) + node.radius + 3;
        }
      }

      animFrameId = requestAnimationFrame(render);
    }

    render();

    // Pan & zoom
    let isPanning = false;
    let panStart = { x: 0, y: 0 };
    let worldStart = { x: 0, y: 0 };

    const canvas = app.canvas as HTMLCanvasElement;

    canvas.addEventListener('pointerdown', (e) => {
      isPanning = true;
      panStart = { x: e.clientX, y: e.clientY };
      worldStart = { x: world.x, y: world.y };
    });

    canvas.addEventListener('pointermove', (e) => {
      if (!isPanning) return;
      const dx = e.clientX - panStart.x;
      const dy = e.clientY - panStart.y;
      world.x = worldStart.x + dx;
      world.y = worldStart.y + dy;
    });

    canvas.addEventListener('pointerup', () => {
      isPanning = false;
    });
    canvas.addEventListener('pointerleave', () => {
      isPanning = false;
    });

    canvas.addEventListener(
      'wheel',
      (e) => {
        e.preventDefault();
        const scaleFactor = e.deltaY > 0 ? 0.95 : 1.05;
        const newScale = Math.min(Math.max(world.scale.x * scaleFactor, 0.2), 3);

        const rect = canvas.getBoundingClientRect();
        const px = e.clientX - rect.left;
        const py = e.clientY - rect.top;

        const worldBefore = { x: (px - world.x) / world.scale.x, y: (py - world.y) / world.scale.y };
        world.scale.set(newScale);
        world.x = px - worldBefore.x * newScale;
        world.y = py - worldBefore.y * newScale;
      },
      { passive: false }
    );

    // Touch pinch zoom
    let lastTouchDist = 0;
    let lastTouchCenter = { x: 0, y: 0 };

    canvas.addEventListener('touchstart', (e) => {
      if (e.touches.length === 2) {
        const dx = e.touches[0].clientX - e.touches[1].clientX;
        const dy = e.touches[0].clientY - e.touches[1].clientY;
        lastTouchDist = Math.sqrt(dx * dx + dy * dy);
        lastTouchCenter = {
          x: (e.touches[0].clientX + e.touches[1].clientX) / 2,
          y: (e.touches[0].clientY + e.touches[1].clientY) / 2,
        };
      }
    }, { passive: true });

    canvas.addEventListener('touchmove', (e) => {
      if (e.touches.length === 2) {
        e.preventDefault();
        const dx = e.touches[0].clientX - e.touches[1].clientX;
        const dy = e.touches[0].clientY - e.touches[1].clientY;
        const dist = Math.sqrt(dx * dx + dy * dy);
        const center = {
          x: (e.touches[0].clientX + e.touches[1].clientX) / 2,
          y: (e.touches[0].clientY + e.touches[1].clientY) / 2,
        };

        if (lastTouchDist > 0) {
          const scaleFactor = dist / lastTouchDist;
          const newScale = Math.min(Math.max(world.scale.x * scaleFactor, 0.2), 3);

          const rect = canvas.getBoundingClientRect();
          const px = center.x - rect.left;
          const py = center.y - rect.top;

          const worldBefore = { x: (px - world.x) / world.scale.x, y: (py - world.y) / world.scale.y };
          world.scale.set(newScale);
          world.x = px - worldBefore.x * newScale;
          world.y = py - worldBefore.y * newScale;

          world.x += center.x - lastTouchCenter.x;
          world.y += center.y - lastTouchCenter.y;
        }

        lastTouchDist = dist;
        lastTouchCenter = center;
      }
    }, { passive: false });

    canvas.addEventListener('touchend', () => {
      lastTouchDist = 0;
    }, { passive: true });
  });

  onDestroy(() => {
    destroyed = true;
    if (animFrameId !== null) {
      cancelAnimationFrame(animFrameId);
    }
    if (simulation) {
      simulation.stop();
      simulation = null;
    }
    if (app) {
      app.destroy(true);
      app = null;
    }
  });
</script>

<div bind:this={canvasContainer} class="graph-canvas"></div>

<style>
  .graph-canvas {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;
    touch-action: none;
  }

  .graph-canvas :global(canvas) {
    display: block;
  }
</style>
