<template>
  <div class="semi-circle-clock">
    <div class="clock-container">
      <svg class="clock" viewBox="0 0 200 110">
        <!-- Hour Markers -->
        <g>
          <circle
            v-for="hour in Array.from({ length: 25 }, (x, i) => i )"
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
    </div>
  </div>
</template>

<script>
function polarToCartesian(cx, cy, radius, angleInDegrees) {
  const angleInRadians = ((angleInDegrees - 180) * Math.PI) / 180.0;
  return {
    x: cx + radius * Math.cos(angleInRadians),
    y: cy + radius * Math.sin(angleInRadians),
  };
}

export default {
  name: "SemiCircleClock",
  data() {
    return {
      timeBlocks: [
        {
          start_time: "2024-12-08T00:00:00",
          end_time: "2024-12-08T06:00:00",
          block_type_id: 1,
          title: "Sleeping",
        },
        {
          start_time: "2024-12-08T06:00:00",
          end_time: "2024-12-08T12:00:00",
          block_type_id: 2,
          title: "Morning Routine",
        },
        {
          start_time: "2024-12-08T12:00:00",
          end_time: "2024-12-08T18:00:00",
          block_type_id: 3,
          title: "Work",
        },
      ],
      blockTypes: [
        { id: 1, name: "Sleep", color: { r: 107, g: 111, b: 246 } },
        { id: 2, name: "Morning Routine", color: { r: 255, g: 111, b: 97 } },
        { id: 3, name: "Work", color: { r: 111, g: 255, b: 136 } },
      ],
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
        const blockType = this.blockTypes.find((type) => type.id === block.block_type_id);
        const startAngle = (new Date(block.start_time).getHours() / 24) * 180;
        const endAngle = (new Date(block.end_time).getHours() / 24) * 180;

        const startPoint = polarToCartesian(cx, cy, radius, startAngle);
        const endPoint = polarToCartesian(cx, cy, radius, endAngle);

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
    hourX(hour, radius) {
      const angle = (hour / 24) * 180;
      const { x } = polarToCartesian(100, 100, radius, angle);
      return x;
    },
    hourY(hour, radius) {
      const angle = (hour / 24) * 180;
      const { y } = polarToCartesian(100, 100, radius, angle);
      return y;
    },
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
  width: 100%;
  height: 100%;
  position: relative;
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
</style>
