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

  toJson(): any {
    return {
      r: this.r,
      g: this.g,
      b: this.b,
    };
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

  static fromJsonArray(json: any): BlockType[] {
    if (!json) {
      return [];
    }
    return json.map((blockType: any) => BlockType.fromJson(blockType));
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
      json.start_time,
      json.end_time,
      json.block_type_id,
      json.title,
    );
  }

  static fromJsonArray(json: any): TimeBlock[] {
    if (!json) {
      return [];
    }
    return json.map((block: any) => TimeBlock.fromJson(block));
  }
}

export class HistoryData {
  blocktypes: BlockType[];
  daydata: TimeBlock[];

  constructor(blocktypes: BlockType[], daydata: TimeBlock[]) {
    this.blocktypes = blocktypes;
    this.daydata = daydata;
  }

  static fromJson(json: any): HistoryData {
    let blocktypes = BlockType.fromJsonArray(json.blocktypes);
    let daydata = TimeBlock.fromJsonArray(json.daydata);

    return new HistoryData(blocktypes, daydata);
  }
}

export class NewBlockType {
  name: string;
  color: Color;

  constructor(name: string, color: Color) {
    this.name = name;
    this.color = color;
  }

  static fromJson(json: any): NewBlockType {
    return new NewBlockType(json.name, Color.fromJson(json.color));
  }

  toJson(): any {
    return {
      name: this.name,
      color: this.color.toJson(),
    };
  }
}

export class CurrentData {
  blockTypeId: number;
  currentBlockName: string;

  constructor(blockTypeId: number, currentBlockName: string) {
    this.blockTypeId = blockTypeId;
    this.currentBlockName = currentBlockName;
  }

  static fromJson(json: any): CurrentData {
    return new CurrentData(json.block_type_id, json.current_block_name);
  }

  public toJson(): any {
    return {
      block_type_id: this.blockTypeId,
      current_block_name: this.currentBlockName,
    };
  }
}

export type HomeData = {
  blocktypes: BlockType[];
  daydata: TimeBlock[];
  currentblock: CurrentData;
};

export class Duration {
  secs: number;
  nanos: number;

  constructor(secs: number, nanos: number) {
    this.secs = secs;
    this.nanos = nanos;
  }

  static fromJson(json: any): Duration {
    return new Duration(json.secs, json.nanos);
  }

  toHours(): number {
    return this.secs / 3600;
  }
}

export class Trend {
  day: string;
  timeSpent: Duration;
  blockTypeId: number;

  constructor(day: string, timeSpent: Duration, blockTypeId: number) {
    this.day = day;
    this.timeSpent = timeSpent;
    this.blockTypeId = blockTypeId;
  }

  static fromJson(json: any): Trend {
    let timeSpent = Duration.fromJson(json.time_spent);
    return new Trend(json.day, timeSpent, json.block_type_id);
  }

  static fromJsonArray(json: any): Trend[] {
    if (!json) {
      return [];
    }
    return json.map((trend: any) => Trend.fromJson(trend));
  }
}

export class Analysis {
  percentages: number[];
  trends: Trend[];
  blocktypes: Array<BlockType>;

  constructor(percentages: number[], trends: Trend[], blocktypes: BlockType[]) {
    this.percentages = percentages;
    this.trends = trends;
    this.blocktypes = blocktypes;
  }

  static fromJson(json: any): Analysis {
    const percentages = json.percentages || [];
    const trends = Trend.fromJsonArray(json.trends);
    const blocktypes = BlockType.fromJsonArray(json.blocktypes);

    return new Analysis(percentages, trends, blocktypes);
  }
}

export class Palette {
  name: string;
  accent: string;
  accentHover: string;
  accent2: string;
  bg: string;
  bgDark: string;
  disabledColor: string;

  constructor(
    name: string,
    accent: string,
    accentHover: string,
    accent2: string,
    bg: string,
    bgDark: string,
    disabledColor: string,
  ) {
    this.name = name;
    this.accent = accent;
    this.accentHover = accentHover;
    this.accent2 = accent2;
    this.bg = bg;
    this.bgDark = bgDark;
    this.disabledColor = disabledColor;
  }

  static fromObject(obj: any): Palette {
    return new Palette(
      obj.name,
      obj.accent,
      obj.accentHover,
      obj.accent2,
      obj.bg,
      obj.bgDark,
      obj.disabledColor,
    );
  }

  static fromJson(json: any): Palette {
    return new Palette(
      json.name,
      json.accent,
      json.accent_hover,
      json.accent2,
      json.bg,
      json.bg_dark,
      json.disabled_color,
    );
  }

  public toJson(): any {
    return {
      name: this.name,
      accent: this.accent,
      accent_hover: this.accentHover,
      accent2: this.accent2,
      bg: this.bg,
      bg_dark: this.bgDark,
      disabled_color: this.disabledColor,
    };
  }
}
