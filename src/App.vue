<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
    SunIcon,
    MoonIcon,
    Cog6ToothIcon,
    TrashIcon,
    PlusIcon,
    DocumentDuplicateIcon,
    ArchiveBoxIcon,
    ExclamationTriangleIcon,
} from "@heroicons/vue/24/outline";

interface BarcodeItem {
    code: string;
    name: string;
}

const barcodes = ref<BarcodeItem[]>([]);
const newBarcode = ref("");
const newBarcodeName = ref("");
const isProcessing = ref(false);
const errorMessage = ref("");
const showSettings = ref(false);
const initialDelay = ref(5000);
const barcodeDelay = ref(500);
const currentScanningIndex = ref<number | null>(null);

function saveBarcodes() {
    localStorage.setItem("barcode_list", JSON.stringify(barcodes.value));
}

onMounted(() => {
    const list_str = localStorage.getItem("barcode_list");
    if (list_str) {
        try {
            const list = JSON.parse(list_str);
            if (Array.isArray(list) && list.length) {
                if (typeof list[0] === "string") {
                    barcodes.value = list.map((code: string) => ({
                        code,
                        name: "",
                    }));
                } else {
                    barcodes.value = list;
                }
            }
        } catch (e) {
            console.error("Error parsing barcode list", e);
        }
    }

    const storedTheme = localStorage.getItem("theme");
    if (storedTheme === "dark") {
        isDark.value = true;
        document.documentElement.classList.add("dark");
    } else {
        isDark.value = false;
        document.documentElement.classList.remove("dark");
    }
});

async function addBarcode() {
    if (newBarcode.value.trim() && /^\d+$/.test(newBarcode.value)) {
        barcodes.value.push({
            code: newBarcode.value.trim(),
            name: newBarcodeName.value.trim(),
        });
        newBarcode.value = "";
        newBarcodeName.value = "";
        errorMessage.value = "";
    } else {
        errorMessage.value = "Please enter a valid numeric barcode";
    }
}

function duplicateBarcode(index: number) {
    const item = barcodes.value[index];
    barcodes.value.splice(index + 1, 0, { ...item });
}

function deleteBarcode(index: number) {
    barcodes.value.splice(index, 1);
}

async function processBarcodes() {
    isProcessing.value = true;
    errorMessage.value = "";
    currentScanningIndex.value = null;

    const unlisten = await listen<number>("simulation_progress", (event) => {
        currentScanningIndex.value = event.payload;
    });

    try {
        await invoke("simulate_barcodes", {
            barcodes: barcodes.value.map((b) => b.code),
            initialDelayMs: initialDelay.value,
            barcodeDelayMs: barcodeDelay.value,
        });
    } catch (err) {
        errorMessage.value = `Error: ${err}`;
    } finally {
        isProcessing.value = false;
        currentScanningIndex.value = null;
        unlisten();
    }
}

function clearAllBarcodes() {
    barcodes.value = [];
    localStorage.setItem("barcode_list", "[]");
}

const isDark = ref(false);
function toggleDarkMode() {
    isDark.value = !isDark.value;
    if (isDark.value) {
        document.documentElement.classList.add("dark");
        localStorage.setItem("theme", "dark");
    } else {
        document.documentElement.classList.remove("dark");
        localStorage.setItem("theme", "light");
    }
}
</script>
<template>
    <div
        class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 transition-colors duration-300"
    >
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 pb-12">
             <!-- Header Section -->
            <div class="flex items-center justify-between mb-8">
                <div class="flex items-center gap-4">
                    <img src="/barcoderz-logo.png" alt="Barcoderz Logo" class="h-12 w-auto drop-shadow-md rounded-xl" />
                    <h1
                        class="text-3xl sm:text-4xl font-extrabold text-gray-900 dark:text-white tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-indigo-500 to-purple-600 dark:from-indigo-400 dark:to-purple-400"
                    >
                        Barcoderz
                    </h1>
                </div>
                
                <button
                    @click="toggleDarkMode"
                    class="flex items-center gap-2 px-4 py-2 rounded-xl bg-white/90 dark:bg-gray-800/80 backdrop-blur-sm ring-1 ring-gray-200/50 dark:ring-gray-700 shadow-sm hover:shadow transition-all duration-200"
                >
                    <SunIcon v-if="!isDark" class="h-5 w-5 text-amber-500" />
                    <MoonIcon v-else class="h-5 w-5 text-indigo-400" />
                    <span
                        class="hidden sm:inline text-sm font-medium text-gray-600 dark:text-gray-300"
                    >
                        {{ isDark ? "Light" : "Dark" }}
                    </span>
                </button>
            </div>

            <div class="mb-10">
                <p class="text-lg text-gray-600 dark:text-gray-300 leading-relaxed max-w-2xl">
                    Streamline your workflow by simulating barcode scans directly to your active window.
                </p>
            </div>

            <!-- Settings Modal -->
            <div
                v-if="showSettings"
                class="fixed inset-0 flex items-center justify-center z-50 backdrop-blur-sm"
            >
                <div
                    class="fixed inset-0 bg-black/30"
                    @click="showSettings = false"
                ></div>
                <div
                    class="relative bg-white dark:bg-gray-800 rounded-2xl shadow-2xl p-6 w-full max-w-md ring-1 ring-gray-900/5"
                >
                    <h3
                        class="text-xl font-semibold mb-6 text-gray-900 dark:text-gray-100"
                    >
                        ⚙️ Simulation Settings
                    </h3>
                    <div class="space-y-5">
                        <div>
                            <label
                                class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
                            >
                                Initial Delay (ms)
                            </label>
                            <input
                                type="number"
                                v-model.number="initialDelay"
                                class="w-full px-4 py-2.5 rounded-lg bg-gray-50 dark:bg-gray-700 border-0 ring-1 ring-gray-200 dark:ring-gray-700 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 transition-all"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
                            >
                                Barcode Delay (ms)
                            </label>
                            <input
                                type="number"
                                v-model.number="barcodeDelay"
                                class="w-full px-4 py-2.5 rounded-lg bg-gray-50 dark:bg-gray-700 border-0 ring-1 ring-gray-200 dark:ring-gray-700 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 transition-all"
                            />
                        </div>
                    </div>
                    <div class="mt-8 flex justify-end gap-3">
                        <button
                            @click="showSettings = false"
                            class="px-4 py-2.5 text-sm font-medium text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700/50 rounded-lg transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            @click="showSettings = false"
                            class="px-4 py-2.5 bg-indigo-500 hover:bg-indigo-600 text-white font-medium rounded-lg transition-colors shadow-sm"
                        >
                            Save Changes
                        </button>
                    </div>
                </div>
            </div>

            <div
                class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm rounded-2xl shadow-xl ring-1 ring-gray-900/5 p-6 sm:p-8"
            >
                <div class="mb-8">
                    <form @submit.prevent="addBarcode" class="flex flex-col sm:flex-row gap-3">
                        <div class="flex-1">
                            <input
                                v-model="newBarcodeName"
                                type="text"
                                placeholder="Barcode Name (Optional)"
                                class="w-full px-4 py-3.5 bg-gray-50 dark:bg-gray-700 rounded-xl ring-1 ring-gray-200 dark:ring-gray-700 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 placeholder-gray-400 dark:placeholder-gray-500 text-gray-900 dark:text-white transition-all"
                            />
                        </div>
                        <div class="flex-1 relative flex gap-2">
                             <div class="relative flex-1">
                                <input
                                    v-model="newBarcode"
                                    type="text"
                                    placeholder="Enter barcode..."
                                    pattern="\d*"
                                    class="w-full pl-4 pr-12 py-3.5 bg-gray-50 dark:bg-gray-700 rounded-xl ring-1 ring-gray-200 dark:ring-gray-700 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 placeholder-gray-400 dark:placeholder-gray-500 text-gray-900 dark:text-white transition-all"
                                />
                                <button
                                    type="submit"
                                    class="absolute right-2 top-2 p-2 bg-indigo-500 hover:bg-indigo-600 rounded-lg text-white transition-colors"
                                >
                                    <PlusIcon class="h-5 w-5" />
                                </button>
                             </div>
                             
                            <button
                                type="button"
                                @click="showSettings = true"
                                class="p-3.5 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl transition-colors"
                            >
                                <Cog6ToothIcon
                                    class="h-5 w-5 text-gray-600 dark:text-gray-300"
                                />
                            </button>
                        </div>
                    </form>

                    <div
                        v-if="errorMessage"
                        class="mt-4 p-4 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400 rounded-lg flex items-center gap-3"
                    >
                        <ExclamationTriangleIcon
                            class="h-5 w-5 flex-shrink-0"
                        />
                        <span>{{ errorMessage }}</span>
                    </div>
                </div>

                <div v-if="barcodes.length" class="mb-8">
                    <div class="flex items-center justify-between mb-4">
                        <h2
                            class="text-lg font-semibold text-gray-900 dark:text-white"
                        >
                            Barcodes to Scan
                        </h2>
                        <span class="text-sm text-gray-500 dark:text-gray-400"
                            >{{ barcodes.length }} items</span
                        >
                    </div>
                    <div class="space-y-2">
                        <div
                            v-for="(item, index) in barcodes"
                            :key="index"
                            class="group flex items-center justify-between py-2 px-4 bg-white dark:bg-gray-700/50 rounded-xl ring-1 ring-gray-200 dark:ring-gray-700 hover:ring-indigo-500 dark:hover:ring-indigo-400 transition-all"
                            :class="{'ring-2 ring-indigo-500 dark:ring-indigo-400': index === currentScanningIndex}"
                        >
                            <div class="flex items-center gap-3 overflow-hidden">
                                <svg
                                    v-if="index === currentScanningIndex"
                                    class="animate-spin h-5 w-5 text-indigo-500"
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                >
                                    <circle
                                        class="opacity-25"
                                        cx="12"
                                        cy="12"
                                        r="10"
                                        stroke="currentColor"
                                        stroke-width="4"
                                    ></circle>
                                    <path
                                        class="opacity-75"
                                        fill="currentColor"
                                        d="M4 12a8 8 0 018-8v8H4z"
                                    ></path>
                                </svg>
                                <div class="flex flex-col min-w-0">
                                    <span v-if="item.name" class="text-sm font-medium text-gray-900 dark:text-white truncate">
                                        {{ item.name }}
                                    </span>
                                    <span class="font-mono text-gray-700 dark:text-gray-300 text-sm">
                                        {{ item.code }}
                                    </span>
                                </div>
                            </div>
                            <div
                                class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity ml-2"
                            >
                                <button
                                    @click="duplicateBarcode(index)"
                                    class="p-2 text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg transition-colors"
                                    title="Duplicate"
                                >
                                    <DocumentDuplicateIcon class="h-5 w-5" />
                                </button>
                                <button
                                    @click="deleteBarcode(index)"
                                    class="p-2 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
                                    title="Delete"
                                >
                                    <TrashIcon class="h-5 w-5" />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div
                    class="flex flex-col sm:flex-row gap-3 pt-6 border-t border-gray-200 dark:border-gray-700"
                >
                    <button
                        @click="processBarcodes"
                        :disabled="barcodes.length === 0 || isProcessing"
                        class="flex-1 flex items-center justify-center gap-2 px-6 py-3.5 bg-indigo-500 hover:bg-indigo-600 text-white font-medium rounded-xl transition-all disabled:opacity-50 disabled:pointer-events-none shadow-sm"
                    >
                        <svg
                            v-if="isProcessing"
                            class="animate-spin h-5 w-5 text-white"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <circle
                                class="opacity-25"
                                cx="12"
                                cy="12"
                                r="10"
                                stroke="currentColor"
                                stroke-width="4"
                            ></circle>
                            <path
                                class="opacity-75"
                                fill="currentColor"
                                d="M4 12a8 8 0 018-8v8H4z"
                            ></path>
                        </svg>
                        <span>{{
                            isProcessing ? "Processing..." : "Start Simulation"
                        }}</span>
                    </button>

                    <div class="flex gap-3">
                        <button
                            @click="saveBarcodes"
                            class="px-6 py-3.5 bg-purple-500 hover:bg-purple-600 text-white font-medium rounded-xl transition-colors shadow-sm flex items-center gap-2"
                        >
                            <ArchiveBoxIcon class="h-5 w-5" />
                            <span class="hidden sm:inline">Save</span>
                        </button>
                        <button
                            @click="clearAllBarcodes"
                            class="px-6 py-3.5 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 font-medium rounded-xl transition-colors flex items-center gap-2"
                        >
                            <TrashIcon class="h-5 w-5" />
                            <span class="hidden sm:inline">Clear</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
