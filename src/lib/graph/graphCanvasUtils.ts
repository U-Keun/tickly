import type { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import type { GraphData } from '$lib/api/graphApi';
import type { Graphics, Text } from 'pixi.js';

export interface SimNode extends SimulationNodeDatum {
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

export interface SimLink extends SimulationLinkDatum<SimNode> {
  source: SimNode;
  target: SimNode;
}

export interface GraphThemeColors {
  paper: string;
  sky: string;
  skyStrong: string;
  mint: string;
  peach: string;
  ink: string;
  inkMuted: string;
  canvas: string;
}

export const TAP_THRESHOLD = 6;

export function hexToNum(hex: string): number {
  return parseInt(hex.replace('#', ''), 16);
}

export function truncateLabel(label: string, maxLen: number): string {
  if (label.length <= maxLen) return label;
  return label.slice(0, maxLen - 1) + '…';
}

function getCssVar(name: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

export function readGraphThemeColors(): GraphThemeColors {
  return {
    paper: getCssVar('--color-paper') || '#f8f7f3',
    sky: getCssVar('--color-accent-sky-strong') || '#8ea9cf',
    skyStrong: getCssVar('--color-accent-sky') || '#a8bddb',
    mint: getCssVar('--color-accent-mint') || '#bfd9c8',
    peach: getCssVar('--color-accent-peach') || '#e9c1ad',
    ink: getCssVar('--color-ink') || '#5b5852',
    inkMuted: getCssVar('--color-ink-muted') || '#7a776f',
    canvas: getCssVar('--color-canvas') || '#f2efe8',
  };
}

export function buildSimulationData(data: GraphData, width: number, height: number): {
  simNodes: SimNode[];
  simLinks: SimLink[];
} {
  const radiusMap = { category: 12, tag: 10, item: 8 };

  const simNodes: SimNode[] = data.nodes.map((node) => ({
    id: `${node.node_type}-${node.id}`,
    rawId: node.id,
    nodeType: node.node_type,
    label: node.label,
    categoryId: node.category_id,
    done: node.done,
    radius: radiusMap[node.node_type] ?? 9,
    graphics: null,
    textObj: null,
    x: width / 2 + (Math.random() - 0.5) * 80,
    y: height / 2 + (Math.random() - 0.5) * 80,
  }));

  const nodeMap = new Map<string, SimNode>();
  for (const node of simNodes) {
    nodeMap.set(node.id, node);
  }

  const simLinks: SimLink[] = [];
  for (const edge of data.edges) {
    const source = nodeMap.get(edge.source);
    const target = nodeMap.get(edge.target);
    if (source && target) {
      simLinks.push({ source, target });
    }
  }

  return { simNodes, simLinks };
}
