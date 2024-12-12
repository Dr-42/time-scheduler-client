<template>
	<div class="container">
		<navbar />
		<status 
			:username="username" 
			:currentStart="cards[cards.length -1].endTime" 
			:currentName="currentData.name" 
			:currentColor="currentData.color"
		/>
		<semi-clock />
		<time-cards 
			:cardData="cards"
		/>
	</div>
</template>

<script lang="ts">
import TimeCards from './components/TimeCards.vue';
import SemiClock from './components/SemiClock.vue';
import Status from './components/Status.vue';
import Navbar from './components/Navbar.vue';

export default {
	components: {
		Navbar,
		TimeCards,
		SemiClock,
		Status,
	},
	data() {
		return {
			username: "Spandan",
			cards: [
				{
					startTime: "2024-12-11T15:48:23.824689862+00:00",
					endTime: "2024-12-11T16:46:23.824689862+00:00",
					blockname: "Amigo",
					color: "#ee22ff",
				},
				{
					startTime: "2024-12-11T16:46:23.824689862+00:00",
					endTime: "2024-12-11T17:52:13.824689862+00:00",
					blockname: "Hola",
					color: "#ee00aa",
				},
			],
			currentData: {
				name: "Eureka",
				color: "#aa0011",
			}
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
	margin: 0 auto;
}
</style>
