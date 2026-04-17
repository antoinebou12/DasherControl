<template>
  <!--
    Workspace tile host (vue-grid-layout v3 beta). GridStack is the main alternative
    for stronger touch/resize UX; a switch would need a compatibility layer for saved layouts.
  -->
  <GridLayout
    :layout="layout"
    :col-num="colNum"
    :row-height="rowHeight"
    :is-draggable="isDraggable"
    :is-resizable="isResizable"
    :responsive="responsive"
    :vertical-compact="verticalCompact"
    :use-css-transforms="useCssTransforms"
    :margin="margin"
    @update:layout="$emit('update:layout', $event)"
  >
    <slot />
  </GridLayout>
</template>

<script setup lang="ts">
import { GridLayout } from "vue-grid-layout";

withDefaults(
  defineProps<{
    layout: Record<string, unknown>[];
    colNum?: number;
    rowHeight?: number;
    isDraggable?: boolean;
    isResizable?: boolean;
    responsive?: boolean;
    verticalCompact?: boolean;
    useCssTransforms?: boolean;
    margin?: [number, number];
  }>(),
  {
    colNum: 12,
    rowHeight: 24,
    isDraggable: true,
    isResizable: true,
    responsive: true,
    verticalCompact: true,
    useCssTransforms: true,
    margin: () => [1, 1],
  },
);

defineEmits<{ "update:layout": [Record<string, unknown>[]] }>();
</script>
