<template>
	<div>
		<time-card
			:blockname="blname"
			:startTime="startTime"
			:endTime="endTime"
			:color="color"
		></time-card>
		<semi-clock></semi-clock>
	</div>
</template>

<script lang="ts">
import TimeCard from './components/TimeCard.vue';
import SemiClock from './components/SemiClock.vue';

export default {
	components : {
		TimeCard,
		SemiClock,
	},
	data() {
	  return {
		startTime: "2024-12-10T15:48:23.824689862+00:00",
		endTime : "2024-12-10T16:46:23.824689862+00:00",
		blname: "Hola",
		color: "#2e2e2e"
	  };
	},
}
</script>
