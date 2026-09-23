/* eslint-disable @typescript-eslint/no-inferrable-types */
/* eslint-disable @typescript-eslint/consistent-type-definitions */

import { useNodesData, type XYPosition } from "@xyflow/svelte";

// import type Graph from "$lib/components/graph.svelte";

// RAW JSON
interface DataValueJsonDto {
  type: DataType;
  value: DataValueJson;
}

interface InputFieldJsonDto {
  name: string;
  data_type: DataType;
  value: DataValueJsonDto;
}

interface OutputPinJsonDto {
  name: string;
  data_type: DataType;
  value: DataValueJsonDto;
}

interface NodeJsonDto {
  id: number;
  name: string;
  kind: string;

  position: [number, number];

  inputs: InputFieldJsonDto[];
  outputs: OutputPinJsonDto[];
}

interface ConnectionJsonDto {
  from: number;
  to: number;
}

export interface GraphJsonDto {
  id: number;

  nodes: NodeJsonDto[];
  connections: ConnectionJsonDto[];
}

// FRONTEND
export class DataValueDto {
  type: DataType = DataType.Number;
  value: DataValue = { type: DataType.Number, value: 0.0 };
}

export class InputFieldDto {
  name: string = "Input";
  dataType: DataType = DataType.Number;
  value: DataValueDto = new DataValueDto();
}

export class OutputPinDto {
  name: string = "Output";
  dataType: DataType = DataType.Number;
  value: DataValueDto = new DataValueDto();
}

export class NodeDto {
  id: string = "0";
  name: string = "Node Name";
  position: XYPosition = { x: 0, y: 0 };
  type = "graph-node";
  dragHandle = ".drag-handle";
  data: NodeDataDto = { kind: "Node Kind Placeholder", name: "Node Name Placeholder", inputs: [], outputs: [] };
}

export class NodeDataDto {
  name: string = "Node Name";
  kind: string = "NodeKind";
  inputs: InputFieldDto[] = [];
  outputs: OutputPinDto[] = [];
}

export class ConnectionDto {
  from: number = 0;
  to: number = 0;
}

export class GraphDto {
  id: number = 0;
  nodes: NodeDto[] = [];
  connections: ConnectionDto[] = [];
}

function json_to_value_dto(json: DataValueJsonDto): DataValueDto {
  const dataValue = new DataValueDto();
  dataValue.type = json.type;

  switch (dataValue.type) {
    case DataType.Number:
      dataValue.value = {
        type: DataType.Number,
        value: json.value as number,
      };
      break;
    case DataType.Boolean:
      dataValue.value = {
        type: DataType.Boolean,
        value: json.value as boolean,
      };
      break;
    case DataType.Color: {
      const values = json.value.e as [number, number, number, number];
      dataValue.value = {
        type: DataType.Color,
        value: { r: values[0], g: values[1], b: values[2], a: values[3] },
      };
      break;
    }
    case DataType.Vector3: {
      const values = json.value.e as [number, number, number];
      dataValue.value = {
        type: DataType.Vector3,
        value: { x: values[0], y: values[1], z: values[2] },
      };
      break;
    }
    case DataType.Point3: {
      const values = json.value.e as [number, number, number];
      dataValue.value = {
        type: DataType.Point3,
        value: { x: values[0], y: values[1], z: values[2] },
      };
      break;
    }
  }

  return dataValue;
}

export function graph_json_to_dto(json: GraphJsonDto): GraphDto {
  const outGraph = new GraphDto();

  outGraph.id = json.id;

  // Connections
  for (const connectionJson of json.connections) {
    const connectionDto = new ConnectionDto();
    connectionDto.from = connectionJson.from;
    connectionDto.to = connectionJson.to;
    outGraph.connections.push(connectionDto);
  }

  for (const nodeJson of json.nodes) {
    const nodeDto = new NodeDto();
    nodeDto.id = nodeJson.id.toString();
    nodeDto.name = nodeJson.name;
    nodeDto.position = { x: nodeJson.position[0], y: nodeJson.position[1] };

    for (const inputJson of nodeJson.inputs) {
      const inputDto = new InputFieldDto();
      inputDto.name = inputJson.name;
      inputDto.dataType = inputJson.data_type;
      inputDto.value = json_to_value_dto(inputJson.value);

      nodeDto.data.inputs.push(inputDto);
    }

    for (const outputJson of nodeJson.outputs) {
      const outputDto = new OutputPinDto();
      outputDto.name = outputJson.name;
      outputDto.dataType = outputJson.data_type;
      outputDto.value = json_to_value_dto(outputJson.value);

      nodeDto.data.outputs.push(outputDto);
    }

    nodeDto.data.kind = nodeJson.kind;
    nodeDto.data.name = nodeDto.name;

    outGraph.nodes.push(nodeDto);
  }

  return outGraph;
}

export enum DataType {
  Number = "Number",
  Boolean = "Boolean",
  Color = "Color",
  Vector3 = "Vector3",
  Point3 = "Point3",
}

export type Color = { r: number; g: number; b: number; a: number };
export type Vec3 = { x: number; y: number; z: number };
export type Point3 = Vec3;

export type DataValue =
  | { type: DataType.Number; value: number }
  | { type: DataType.Boolean; value: boolean }
  | { type: DataType.Color; value: Color }
  | { type: DataType.Vector3; value: Vec3 }
  | { type: DataType.Point3; value: Point3 };

type DataValueJson =
  | number // Number
  | boolean // Boolean
  | { e: [number, number, number, number] } // Color
  | { e: [number, number, number] } // Vector3
  | { e: [number, number, number] }; // Point3
