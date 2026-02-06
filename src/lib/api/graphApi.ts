import { invoke } from './client';

export interface GraphNode {
  id: number;
  node_type: 'item' | 'tag' | 'category';
  label: string;
  category_id: number | null;
  done: boolean | null;
}

export interface GraphEdge {
  source: string;
  target: string;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export async function getGraphData(): Promise<GraphData> {
  return invoke<GraphData>('get_graph_data');
}
