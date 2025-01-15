<template>
	<div class="home">
		<error-display
			v-if="error"
			:errorText="errorText"
		/>
		<div id="loading" v-if="loading">
			<loading-spinner />
		</div>
		<div id="content" v-else>
			<div class="header">
				<status 
					:username="username" 
					:currentStart="currentStart" 
					:currentName="currentName" 
					:currentColor="currentColor"
				/>
				<semi-clock
					:timeBlocks="cards"
					:blockTypes="blockTypes"
					:currentBlock="currentData"
				/>
			</div>
			<div class="time-cards">
				<time-cards 
					:cardData="cards"
					:blockTypes="blockTypes"
					@split-block="openSplitBlockModal"
				/>
			</div>
		</div>

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
		<split-block-modal
			v-if="currentModal === 'splitBlock'"
			:blockTypes="blockTypes"
			:timeblock="currentActionBlock!"
			@close="currentModal = null"
			@done="handleSplitBlock"
		/>
	</div>
</template>

<script lang="ts">
import TimeCards from '../components/subviews/TimeCards.vue';
import SemiClock from '../components/subviews/SemiClock.vue';
import Status from '../components/subviews/Status.vue';
import ErrorDisplay from '../components/subviews/ErrorDisplay.vue';
import LoadingSpinner from '../components/subviews/LoadingSpinner.vue';

import NextBlockModal from '../components/modals/NextBlockModal.vue';
import ChangeBlockModal from '../components/modals/ChangeBlockModal.vue';
import AddBlocktypeModal from '../components/modals/AddBlocktypeModal.vue';
import SplitBlockModal from '../components/modals/SplitBlockModal.vue';

import ChevronRightBoxIcon from "vue-material-design-icons/ChevronRightBox.vue";
import SwapHorizontalCircleIcon from "vue-material-design-icons/SwapHorizontalCircle.vue";
import PlusBoxIcon from "vue-material-design-icons/PlusBox.vue";

import { TimeBlock, BlockType, CurrentData, NewBlockType, Color, HomeData } from '../types';
import { invoke } from '@tauri-apps/api/core';

type MetaData = {
	username: string;
}

type SplitBlockModalData = {
  splitTime: string;
  beforeTitle: string;
  beforeBlockType: number;
  afterTitle: string;
  afterBlockType: number;
};

export default {
	components: {
		TimeCards,
		SemiClock,
		Status,
		ErrorDisplay,
		LoadingSpinner,

		NextBlockModal,
		ChangeBlockModal,
		AddBlocktypeModal,
		SplitBlockModal,

		ChevronRightBoxIcon,
		SwapHorizontalCircleIcon,
		PlusBoxIcon
	},
	computed: {
		currentColor() {
			if (!this.currentData) {
				return new Color(0, 0, 0);
			} else {
				let currentBlock = this.blockTypes.find(block => block.id === this.currentData?.blockTypeId);
				if (currentBlock) {
					return currentBlock.color;
				} else {
					return new Color(0, 0, 0);
				}
			}
		},
		currentName() : string {
			if (!this.currentData) {
				return "No current block";
			}
			return this.currentData.currentBlockName;
		},
		currentStart() {
			if (this.cards.length === 0) {
				let today = new Date();
				today.setHours(0, 0, 0, 0);
				return today.toString();
			} else {
				return this.cards[0].endTime;
			}
		},
	},
	data() {
		return {
			username: "Spandan",
			cards: [] as TimeBlock[],
			currentData: null as CurrentData | null,
			currentModal: null as string | null,
			blockTypes: [] as BlockType[],
			error: false,
			errorText: {},
			loading: true,
			currentActionBlock: null as TimeBlock | null
		};
	},
	methods: {
		openModal(type: string) {
			this.currentModal = type;
		},
		async handleNextBlock(data : CurrentData) {
			try {
				await invoke("post_next_block", { data : data.toJson() });
				this.currentModal = null;
				window.location.reload();
			} catch (e) {
				console.error(e);
				this.error = true;
				this.errorText = e as string;
			}
		},
		async handleChangeBlock(data : CurrentData) {
			try {
				await invoke("post_change_current", { data : data.toJson() });
				this.currentModal = null;
				window.location.reload();
			} catch (e) {
				console.error(e);
				this.error = true;
				this.errorText = e as string;
			}
		},
		async handleAddBlockType(data: NewBlockType) {
			console.log(data);
			try {
				await invoke("post_new_block_type", { data : data.toJson() });
				this.currentModal = null;
				window.location.reload();
			} catch (e) {
				console.error(e);
				this.error = true;
				this.errorText = e as string;
			}
		},
		openSplitBlockModal(block: any) {
			this.currentActionBlock = TimeBlock.fromObject(block);
			this.currentModal = "splitBlock";
		},
		async handleSplitBlock(data: SplitBlockModalData) {
			try {
				const sendData = {
					start_time: this.currentActionBlock!.startTime,
					end_time: this.currentActionBlock!.endTime,
					split_time: data.splitTime,
					before_title: data.beforeTitle,
					before_block_type_id: data.beforeBlockType,
					after_title: data.afterTitle,
					after_block_type_id: data.afterBlockType,
				};
				await invoke("post_split_block", { data : sendData });
				this.currentActionBlock = null;
				this.currentModal = null;
				window.location.reload();
			} catch (e) {
				console.error(e);
				this.error = true;
				this.errorText = e as string;
			}
		}
	},
	async mounted() {
		try {
			this.loading = true;
			let home_data: HomeData = await invoke("get_home_data");
			this.cards = TimeBlock.fromJsonArray(home_data.daydata);
			this.currentData = CurrentData.fromJson(home_data.currentblock);
			this.blockTypes = BlockType.fromJsonArray(home_data.blocktypes);

			let meta = await invoke("get_meta");
			let meta_data = meta as MetaData;
			this.username = meta_data.username;
			this.loading = false;
		} catch (e) {
			console.error(e);
			this.error = true;
			this.errorText = e as string;
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
	background-color: var(--bg);
	margin: 0;
	height: calc(100vh - 50px);
	padding: 0;
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
	background-color: var(--accent);
	color: white;
	border: 2px solid var(--accent2);
	border-radius: 25%;
	width: 56px;
	height: 56px;
	display: flex;
	justify-content: center;
	align-items: center;
	box-shadow: 0px 4px 10px var(--bg-dark);
	cursor: pointer;
	outline: none;
}

.fab:hover {
	background-color: var(--accent-hover);
}

.fab svg {
	width: 24px;
	height: 24px;
}

.time-cards {
	overflow-y: scroll;
	height: calc(55vh - 25px);
}

.header {
	height: calc(45vh - 25px);
}

#loading {
  display: flex;
  justify-content: center;
  align-items: center;
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  height: calc(100vh - (50px + 10vh));
  background-color: var(--bg);
  z-index: 3;
}
</style>
