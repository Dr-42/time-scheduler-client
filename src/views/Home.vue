<template>
	<div class="home">
		<status 
			:username="username" 
			:currentStart="cards[cards.length -1].endTime" 
			:currentName="currentData.name" 
			:currentColor="blockTypes.find(block => block.id === currentData.id)?.color"
		/>
		<semi-clock
			:timeBlocks="cards"
			:blockTypes="blockTypes"
		/>
		<time-cards 
			:cardData="cards"
			:blockTypes="blockTypes"
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
		<!-- Modals -->
		<next-block-modal
			v-if="currentModal === 'nextBlock'"
			:blockTypes="blockTypes"
			@close="currentModal = null"
			@done="handleNextBlock"
		/>
		<change-block-modal
			v-if="currentModal === 'changeBlock'"
			:blockTypes="blockTypes"
			:currentData="currentData"
			@close="currentModal = null"
			@done="handleChangeBlock"
		/>
		<add-blocktype-modal
			v-if="currentModal === 'addBlockType'"
			@close="currentModal = null"
			@done="handleAddBlockType"
		/>
	</div>
</template>

<script lang="ts">
import TimeCards from '../components/TimeCards.vue';
import SemiClock from '../components/SemiClock.vue';
import Status from '../components/Status.vue';

import NextBlockModal from '../components/NextBlockModal.vue';
import ChangeBlockModal from '../components/ChangeBlockModal.vue';
import AddBlocktypeModal from '../components/AddBlocktypeModal.vue';

import ChevronRightBoxIcon from "vue-material-design-icons/ChevronRightBox.vue";
import SwapHorizontalCircleIcon from "vue-material-design-icons/SwapHorizontalCircle.vue";
import PlusBoxIcon from "vue-material-design-icons/PlusBox.vue";

type Color = {
	r: number;
	g: number;
	b: number;
}

type BlockType = {
	id: number;
	name: string;
	color: Color;
}

type TimeBlock = {
  blockname: string;
  startTime: string;
  endTime: string;
  blockId: number;
}

type NextBlockSubmitData = {
	name: string;
	id: number;
}

type NewBlockType = {
	name: string;
	color: Color;
}


export default {
	components: {
		TimeCards,
		SemiClock,
		Status,

		NextBlockModal,
		ChangeBlockModal,
		AddBlocktypeModal,

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
					blockId: 2,
				},
				{
					startTime: "2024-12-11T16:46:23.824689862+00:00",
					endTime: "2024-12-11T17:52:13.824689862+00:00",
					blockname: "Hola",
					blockId: 1,
				},
			] as TimeBlock[],
			currentData: {
				name: "Eureka",
				id: 2,
			},
			currentModal: null as string | null,
			blockTypes: [
				{ id: 1, name: "Work", color: { r: 255, g: 0, b: 0 } },
				{ id: 2, name: "Exercise", color: { r: 0, g: 255, b: 0 } },
				{ id: 3, name: "Study", color: { r: 0, g: 0, b: 255 } },
			] as BlockType[],
		};
	},
	methods: {
		openModal(type: string) {
			this.currentModal = type;
		},
		handleNextBlock(data : NextBlockSubmitData) {
			console.log("Next Block submitted:", { data });
			this.currentModal = null;
		},
		handleChangeBlock(data : NextBlockSubmitData) {
			console.log("Change Block submitted:", { data });
			this.currentModal = null;
		},
		handleAddBlockType(data: NewBlockType) {
			console.log("Add Block Type submitted:", { data });
			this.currentModal = null;
		},
	},
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
