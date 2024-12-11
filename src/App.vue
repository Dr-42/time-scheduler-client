<template>
	<div class="container">
		<status 
			:username="username" 
			:currentStart="startTime" 
			:currentName="blname" 
			:currentColor="color"
		/>
		<semi-clock />
		<time-card 
			:blockname="blname" 
			:startTime="startTime" 
			:endTime="endTime" 
			:color="color" 
		/>
	</div>
</template>

<script lang="ts">
import TimeCard from './components/TimeCard.vue';
import SemiClock from './components/SemiClock.vue';
import Status from './components/Status.vue';

export default {
	components: {
		TimeCard,
		SemiClock,
		Status,
	},
	data() {
		return {
			username: "Spandan",
			startTime: "2024-12-10T15:48:23.824689862+00:00",
			endTime: "2024-12-10T16:46:23.824689862+00:00",
			blname: "Hola",
			color: "#ee22ff",
		};
	},
}
</script>

<style scoped>
.container {
	font-family: Avenir, Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-align: center;
	color: #2c3e50;
	background-color: #2e2e2e;
}
</style>
