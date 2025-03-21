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
          :d="block.path"
          :stroke="block.color"
          stroke-width="8"
          fill="none"
        />

        <!-- Current Block -->
        <path
          v-if="currentRenderedBlock"
          :d="currentRenderedBlock.path"
          :stroke="currentRenderedBlock.color"
          stroke-width="8"
          fill="none"
        />

        <!-- Sunrise Icon -->
        <image
          href="../../assets/sunrise.svg"
          :x="hourX(sunriseHour , 80) - 7"
          :y="hourY(sunriseHour , 80) - 7"
          height="14"
          width="14"
        />

        <!-- Sunset Icon -->
        <image
          href="../../assets/sunset.svg"
          :x="hourX(sunsetHour , 80) - 7"
          :y="hourY(sunsetHour , 80) - 7"
          height="14"
          width="14"
        />
      </svg>
    </div>
    <div class="task-labels">
      <div v-for="block in blockTypes" class="task-label">
        <div class="task-square" :style="{ backgroundColor: block.color.toString() }"></div>
        <div class="task-name">{{ block.name }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { BlockType, CurrentData, TimeBlock } from '../../types';

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
    currentBlock: {
      type: [CurrentData, null],
    },
  },
  data() {
    return {
      sunrise: null as Date | null,
      sunset: null as Date | null,
      timer: null as null | number, // Timer for updating the current block
    };
  },
  computed: {
    sunriseHour() {
      if (!this.sunrise) return 0;
      return new Date(this.sunrise).getHours();
    },
    sunsetHour() {
      if (!this.sunset) return 0;
      return new Date(this.sunset).getHours();
    },
    renderedBlocks() {
      const radius = 60; // Radius of the semi-circle
      const cx = 100; // Center X
      const cy = 100;

      return this.timeBlocks.map((block) => {
        const blockType = this.blockTypes.find(
          (type) => type.id === block.blockTypeId
        );
        if (!blockType) {
          return {
            path: "",
            color: "rgb(255, 255, 255)",
          };
        }
        const startTime = new Date(block.startTime);
        const endTime = new Date(block.endTime);

        const color = blockType.color.toString();
        return this.getBlockArc(startTime, endTime, color, radius, cx, cy);
      });
    },
    currentRenderedBlock() {
      if (!this.currentBlock) return null;

      const radius = 60;
      const cx = 100;
      const cy = 100;

      let midnight = new Date();
      midnight.setHours(0, 0, 0, 0);
      const startTime =
        this.timeBlocks.length > 0 ? new Date(this.timeBlocks[0].endTime): midnight;
      const endTime = new Date();

      const blockType = this.blockTypes.find(
        (type) => type.id === this.currentBlock!.blockTypeId
      );
      if (!blockType) {
        return null;
      }
      const color = blockType.color.toString();
      return this.getBlockArc(startTime, endTime, color, radius, cx, cy);
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
    updateCurrentBlock() {
      // Forces recomputation of computed properties
      this.$forceUpdate();
    },
    async getSunHours() {
      try {
        let sunhours = await invoke("get_sun_hours") as any;
        this.sunrise = new Date(sunhours.sunrise);
        this.sunset = new Date(sunhours.sunset);
      } catch (e) {
        console.error(e);
      }
    },
    getBlockArc(startTime: Date, endTime: Date, color: string, radius: number, cx: number, cy: number) {
      const startHours = startTime.getHours() + startTime.getMinutes() / 60;
      const endHours = endTime.getHours() + endTime.getMinutes() / 60;

      const startAngle = (startHours / 24.0) * 180;
      const endAngle = (endHours / 24.0) * 180;

      const startPoint = this.polarToCartesian(cx, cy, radius, startAngle);
      const endPoint = this.polarToCartesian(cx, cy, radius, endAngle);

      const largeArcFlag = endAngle - startAngle <= 180 ? "0" : "1";
      const path = `
M ${startPoint.x} ${startPoint.y}
A ${radius} ${radius} 0 ${largeArcFlag} 1 ${endPoint.x} ${endPoint.y}
`;
      return {
        path,
        color,
      };
    }
  },
  async mounted() {
    // Start updating the current block every 5 minutes
    this.timer = window.setInterval(this.updateCurrentBlock, 5 * 60 * 1000);
    await this.getSunHours();
  },
  beforeDestroy() {
    // Clear the interval timer to avoid memory leaks
    if (this.timer) {
      clearInterval(this.timer);
    }
  },
};
</script>

<style scoped>
.semi-circle-clock {
  font-family: "Poppins", sans-serif;
  background-color: var(--bg);
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 4px;
  width: 100%;
  box-sizing: border-box;
}

.clock-container {
  position: relative;
  display: flex;
  justify-content: space-around;
  align-items: center;
  flex-direction: row;
  margin-bottom: 10px;
}

.clock {
  width: 100%;
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
  overflow-y: scroll;
  height: 80%;
  margin-right: 5%;
}

.task-label {
  display: flex;
  align-items: center; 
  gap: 8px;
  font-size: 12px; 
  padding: 2px; 
  border-radius: 5px; 
  height: 100%;
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
