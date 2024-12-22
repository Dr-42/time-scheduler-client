type Color = {
  r: number;
  g: number;
  b: number;
};

type BlockType = {
  id: number;
  name: string;
  color: Color;
};

type TimeBlock = {
  startTime: string;
  endTime: string;
  blockTypeId: number;
  title: string;
};

type NewBlockType = {
  name: string;
  color: Color;
};

type CurrentData = {
  blockTypeId: number;
  currentBlockName: string;
};
