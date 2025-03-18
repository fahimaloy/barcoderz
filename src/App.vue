<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import trashIcon from "./assets/trash.svg?raw";
import settingsIcon from "./assets/settings.svg?raw";
const barcodes = ref<string[]>([]);
const newBarcode = ref("");
const isProcessing = ref(false);
const errorMessage = ref("");
const showSettings = ref(false);
const initialDelay = ref(5000);
const barcodeDelay = ref(500);
watch(barcodes, (newCodes, oldCodes) => {
    const str = JSON.stringify(newCodes);
    localStorage.setItem("barcode_list", str);
});
const clearAllBarcodes = () => {
    barcode.value = [];
};
onMounted(() => {
    const list_str = localStorage.getItem("barcode_list") || null;
    if (list_str) {
        try {
            const list = JSON.parse(list_str);
            if (list && list?.length) {
                barcode.value = list;
            }
        } catch {}
    }
});
async function addBarcode() {
    if (newBarcode.value.trim() && /^\d+$/.test(newBarcode.value)) {
        barcodes.value.push(newBarcode.value.trim());
        newBarcode.value = "";
        errorMessage.value = "";
    } else {
        errorMessage.value = "Please enter a valid numeric barcode";
    }
}

function duplicateBarcode(index: number) {
    barcodes.value.splice(index + 1, 0, barcodes.value[index]);
}

function deleteBarcode(index: number) {
    barcodes.value.splice(index, 1);
}

async function processBarcodes() {
    isProcessing.value = true;
    errorMessage.value = "";

    try {
        await invoke("simulate_barcodes", {
            barcodes: barcodes.value,
            initialDelayMs: initialDelay.value,
            barcodeDelayMs: barcodeDelay.value,
        });
    } catch (err) {
        errorMessage.value = `Error: ${err}`;
    } finally {
        isProcessing.value = false;
    }
}
</script>

<template>
    <div class="container">
        <!-- Settings Modal -->
        <div
            v-if="showSettings"
            class="modal-overlay"
            @click.self="showSettings = false"
        >
            <div class="modal">
                <h3>Simulation Settings</h3>

                <div class="setting-item">
                    <label>Initial Delay (ms):</label>
                    <input
                        type="number"
                        v-model.number="initialDelay"
                        min="0"
                        step="100"
                    />
                </div>

                <div class="setting-item">
                    <label>Barcode Delay (ms):</label>
                    <input
                        type="number"
                        v-model.number="barcodeDelay"
                        min="0"
                        step="10"
                    />
                </div>

                <div class="modal-buttons">
                    <button
                        class="btn cancel-btn"
                        @click="showSettings = false"
                    >
                        Cancel
                    </button>
                    <button class="btn save-btn" @click="showSettings = false">
                        Save
                    </button>
                </div>
            </div>
        </div>

        <!-- Main Content -->
        <form @submit.prevent="addBarcode" class="input-group">
            <input
                v-model="newBarcode"
                type="text"
                placeholder="Enter barcode..."
                pattern="\d*"
                class="barcode-input"
            />
            <button type="submit" class="btn plus-btn">+ Add</button>

            <!-- Settings Button -->
            <button class="settings-btn" @click="showSettings = true">
                <div class="icon black-icon" v-html="settingsIcon" />
            </button>
        </form>

        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>

        <div v-if="barcodes.length > 0" class="barcode-list">
            <div
                v-for="(barcode, index) in barcodes"
                :key="index"
                class="barcode-item"
            >
                <span class="barcode-text">{{ barcode }}</span>
                <div class="actions">
                    <button
                        @click="duplicateBarcode(index)"
                        class="btn action-btn"
                    >
                        Copy
                    </button>
                    <button
                        @click="deleteBarcode(index)"
                        class="btn action-btn delete"
                    >
                        ×
                    </button>
                </div>
            </div>
        </div>
        <div class="central-action">
            <button
                @click="processBarcodes"
                :disabled="barcodes.length === 0 || isProcessing"
                class="btn proceed-btn"
            >
                {{ isProcessing ? "Processing..." : "Start Simulation" }}
            </button>
            <button class="btn clear-btn">
                <div class="icon clear-icon" v-html="trashIcon" />
            </button>
        </div>
    </div>
</template>

<style scoped>
.central-action {
    display: flex;
    align-items: center;
    gap: 8px;
}
.clear-btn {
    background-color: #b5293e;
}
.clear-icon {
    color: #fff;
}
.container {
    max-width: 600px;
    margin: 1rem auto;
    padding: 1rem;
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    position: relative;
    margin-top: 20px;
}

.input-group {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.barcode-input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    font-size: 0.95rem;
}

.barcode-list {
    margin: 1rem 0;
    border: 1px solid #eee;
    border-radius: 4px;
}

.barcode-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: white;
    border-bottom: 1px solid #eee;
}

.barcode-item:last-child {
    border-bottom: none;
}

.actions {
    display: flex;
    gap: 0.5rem;
}

.btn {
    padding: 0.3rem 0.6rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s ease;
}

.plus-btn {
    background-color: #4caf50;
    color: white;
}

.proceed-btn {
    width: 100%;
    background-color: #2196f3;
    color: white;
    padding: 0.75rem;
}

.action-btn {
    background-color: #f5f5f5;
    color: #666;
    padding: 0.3rem 0.7rem;
}

.delete {
    background-color: #ffebee;
    color: #f44336;
}

.settings-btn {
    border: none;
    border-radius: 5px;
    padding: 0.4rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #fff;
}
.icon {
    width: 30px;
}
.black-icon {
    color: #000;
}
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.3);
    display: grid;
    place-items: center;
}

.modal {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    width: 300px;
}

.setting-item {
    margin: 1rem 0;
}

.setting-item label {
    display: block;
    margin-bottom: 0.5rem;
    color: #666;
}

.setting-item input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.modal-buttons {
    display: flex;
    gap: 0.5rem;
    margin-top: 1.5rem;
}

.save-btn {
    background-color: #4caf50;
    color: white;
}

.cancel-btn {
    background-color: #f5f5f5;
    color: #666;
}

.error-message {
    color: #f44336;
    padding: 0.5rem;
    background: #ffebee;
    border-radius: 4px;
    margin: 0.5rem 0;
}
</style>
