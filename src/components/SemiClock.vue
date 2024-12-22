<template>
  <div class="semi-circle-clock">
    <div class="clock-container">
      <svg class="clock" viewBox="0 0 200 110">
        <!-- Hour Markers -->
        <g>
          <circle
            v-for="hour in Array.from({ length: 25 }, (_, i) => i )"
            :key="hour"
            :cx="hourX(hour, 70)"
            :cy="hourY(hour, 70)"
            r="2"
            fill="#fff"
          />
        </g>

        <!-- Task Band -->
        <path
          v-for="block in renderedBlocks"
          :key="block.id"
          :d="block.path"
          :stroke="block.color"
          stroke-width="8"
          fill="none"
        />
        <!-- Sunrise Icon -->
        <image
          href="https://www.svgrepo.com/show/281245/sunrise-forecast.svg"
          :x="hourX(sunriseHour , 90) - 10"
          :y="hourY(sunriseHour , 90) - 10"
          height="20"
          width="20"
        />

        <!-- Sunset Icon -->
        <image
          href="https://www.svgrepo.com/show/281239/sunset.svg"
          :x="hourX(sunsetHour , 90) - 10"
          :y="hourY(sunsetHour , 90) - 10"
          height="20"
          width="20"
        />
      </svg>
      <div class="task-labels">
        <div v-for="block in blockTypes" class="task-label">
          <div class="task-square" :style="{ backgroundColor: colorToString(block.color) }"></div>
          <div class="task-name">{{ block.name }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: "SemiCircleClock",
  props: {
    timeBlocks: {
      type: Array<TimeBlock>,
      required: true,
    },
    blockTypes: {
      type: Array<BlockType>,
      required: true,
    },
  },
  data() {
    return {
      sunrise: "2024-12-08T07:00:00",
      sunset: "2024-12-08T18:00:00",
    };
  },
  computed: {
    sunriseHour() {
      return new Date(this.sunrise).getHours();
    },
    sunsetHour() {
      return new Date(this.sunset).getHours();
    },
    renderedBlocks() {
      const radius = 60; // Radius of the semi-circle
      const cx = 100; // Center X
      const cy = 100; // Center Y

      return this.timeBlocks.map((block, id) => {
        const blockType = this.blockTypes.find((type) => type.id === block.blockTypeId);
        if (!blockType) {
          return {
            id,
            path: "",
            color: "rgb(255, 255, 255)",
          };
        }
        const startAngle = (new Date(block.startTime).getHours() / 24) * 180;
        const endAngle = (new Date(block.endTime).getHours() / 24) * 180;

        const startPoint = this.polarToCartesian(cx, cy, radius, startAngle);
        const endPoint = this.polarToCartesian(cx, cy, radius, endAngle);

        const largeArcFlag = endAngle - startAngle <= 180 ? "0" : "1";
        const path = `
M ${startPoint.x} ${startPoint.y}
A ${radius} ${radius} 0 ${largeArcFlag} 1 ${endPoint.x} ${endPoint.y}
`;

        return {
          id,
          path,
          color: `rgb(${blockType.color.r}, ${blockType.color.g}, ${blockType.color.b})`,
        };
      });
    },
  },
  methods: {
    polarToCartesian(cx: number, cy: number, radius: number, angleInDegrees: number) {
      const angleInRadians = ((angleInDegrees - 180) * Math.PI) / 180.0;
      return {
        x: cx + radius * Math.cos(angleInRadians),
        y: cy + radius * Math.sin(angleInRadians),
      };
    },
    hourX(hour: number, radius: number) {
      const angle = (hour / 24) * 180;
      const { x } = this.polarToCartesian(100, 100, radius, angle);
      return x;
    },
    hourY(hour: number, radius: number) {
      const angle = (hour / 24) * 180;
      const { y } = this.polarToCartesian(100, 100, radius, angle);
      return y;
    },
    colorToString(color: { r: number; g: number; b: number }) {
      return `rgb(${color.r}, ${color.g}, ${color.b})`;
    }
  },
};
</script>

<style scoped>
.semi-circle-clock {
  font-family: "Poppins", sans-serif;
  background-color: #2e2e2e;
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 4px;
  width: 100%;
  height: 25vh;
  box-sizing: border-box;
}

.clock-container {
  width: 80%;
  position: relative;
  display: flex;
  justify-content: space-around;
  align-items: center;
  flex-direction: row;
}

.clock {
  width: 75%;
  height: 100%;
}

circle {
  opacity: 0.8;
}

path {
  opacity: 0.9;
}

.task-labels {
  display: flex;
  flex-direction: column;
  width: 25%;
  gap: 1px; 
  overflow-y: auto;
}

.task-label {
  display: flex;
  align-items: center; 
  gap: 8px;
  font-size: 12px; 
  padding: 2px; 
  border-radius: 5px; 
}

.task-square {
  width: 14px; 
  height: 14px; 
  border-radius: 3px; 
  flex-shrink: 0;
  flex-grow: 0;
  margin: 0;
}

.task-name {
  overflow-x: hidden;
  overflow-y: hidden;
  text-overflow: ellipsis;
  text-align: left;
  white-space: nowrap; 
  word-wrap: break-word;
  hyphens: auto;
  flex-grow: 1; 
}
</style>
