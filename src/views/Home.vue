<template>
	<div class="home">
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

		<!-- Floating Action Buttons -->
		<div class="floating-buttons">
			<button class="fab" @click="openModal('changeBlock')" title="Change current block">
				<swap-horizontal-circle-icon />
			</button>
			<button class="fab" @click="openModal('addBlockType')" title="Add blocktype">
				<plus-box-icon />
			</button>
			<button class="fab" @click="openModal('nextBlock')" title="Next block">
				<chevron-right-box-icon />
			</button>
		</div>
	</div>
</template>

<script lang="ts">
import TimeCards from '../components/TimeCards.vue';
import SemiClock from '../components/SemiClock.vue';
import Status from '../components/Status.vue';

import ChevronRightBoxIcon from "vue-material-design-icons/ChevronRightBox.vue";
import SwapHorizontalCircleIcon from "vue-material-design-icons/SwapHorizontalCircle.vue";
import PlusBoxIcon from "vue-material-design-icons/PlusBox.vue";


export default {
	components: {
		TimeCards,
		SemiClock,
		Status,

		ChevronRightBoxIcon,
		SwapHorizontalCircleIcon,
		PlusBoxIcon
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
	methods: {
		openModal(type: string) {
			console.log(type);
		}
	}
}
</script>

<style scoped>
.home {
	font-family: Avenir, Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-align: center;
	color: #2c3e50;
	background-color: #2e2e2e;
	margin: 0;
}

.floating-buttons {
	position: fixed;
	bottom: 20px;
	right: 20px;
	display: flex;
	flex-direction: column;
	gap: 10px;
	z-index: 15;
}

.fab {
	background-color: #6e006e;
	color: white;
	border: none;
	border-radius: 25%;
	width: 56px;
	height: 56px;
	display: flex;
	justify-content: center;
	align-items: center;
	box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
	cursor: pointer;
	outline: none;
}

.fab:hover {
	background-color: #3700b3;
}

.fab svg {
	width: 24px;
	height: 24px;
}
</style>
