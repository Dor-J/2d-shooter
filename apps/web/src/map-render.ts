export type MapPolygon = { vertices: [{ x: number, y: number }, { x: number, y: number }, { x: number, y: number }], kind: string }

export function polygonVertexBuffer(polygons: MapPolygon[]): Float32Array {
  return new Float32Array(polygons.flatMap(polygon => polygon.vertices.flatMap(vertex => [vertex.x, vertex.y])))
}
