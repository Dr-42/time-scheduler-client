export class Color {
  r: number;
  g: number;
  b: number;

  constructor(r: number, g: number, b: number) {
    this.r = r;
    this.g = g;
    this.b = b;
  }

  static fromJson(json: any): Color {
    return new Color(json.r, json.g, json.b);
  }

  toString(): string {
    return `rgb(${this.r}, ${this.g}, ${this.b})`;
  }

  contrastColor(): string {
    const brightness = Math.round(
      (this.r * 299 + this.g * 587 + this.b * 114) / 1000,
    );
    return brightness > 125
      ? "rgba(0, 0, 0, 0.99)"
      : "rgba(255, 255, 255, 0.99)";
  }
}

export class BlockType {
  id: number;
  name: string;
  color: Color;

  constructor(id: number, name: string, color: Color) {
    this.id = id;
    this.name = name;
    this.color = color;
  }

  static fromJson(json: any): BlockType {
    return new BlockType(json.id, json.name, Color.fromJson(json.color));
  }
}

export class TimeBlock {
  startTime: string;
  endTime: string;
  blockTypeId: number;
  title: string;

  constructor(
    startTime: string,
    endTime: string,
    blockTypeId: number,
    title: string,
  ) {
    this.startTime = startTime;
    this.endTime = endTime;
    this.blockTypeId = blockTypeId;
    this.title = title;
  }

  static fromJson(json: any): TimeBlock {
    return new TimeBlock(
      json.startTime,
      json.endTime,
      json.blockTypeId,
      json.title,
    );
  }
}

export type NewBlockType = {
  name: string;
  color: Color;
};

export type CurrentData = {
  blockTypeId: number;
  currentBlockName: string;
};
