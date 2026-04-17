import { defineAsyncComponent } from "vue";
import Editor from "./Editor/Editor.vue";
import BorderlessIFrame from "./IFrame/BorderlessIFrame.vue";
import Shortcut from "./Shortcut/Shortcut.vue";
import DockerManager from "./DockerManager/DockerManager.vue";
import Terminal from "./Terminal/Terminal.vue";
import CaddyConfigurator from "./CaddyConfigurator/CaddyConfigurator.vue";
import Canvas from "./Canvas/Canvas.vue";
import SystemMonitor from "./SystemMonitor/SystemMonitor.vue";
import CpuMetrics from "./CpuMetrics/CpuMetrics.vue";
import MemoryMetrics from "./MemoryMetrics/MemoryMetrics.vue";
import DiskMetrics from "./DiskMetrics/DiskMetrics.vue";
import NetworkMetrics from "./NetworkMetrics/NetworkMetrics.vue";
import GpuMetrics from "./GpuMetrics/GpuMetrics.vue";
import Assistant from "./Assistant/Assistant.vue";

import CreateNewSetting from "./CreateNew/CreateNewSetting.vue";
import IFrameSetting from "./IFrame/IFrameSetting.vue";
import ShortcutSetting from "./Shortcut/ShortcutSetting.vue";
import DockerManagerSetting from "./DockerManager/DockerManagerSetting.vue";
import TerminalSetting from "./Terminal/TerminalSetting.vue";
import CanvasSetting from "./Canvas/CanvasSetting.vue";

import { registerApplet } from "./appletRegistryCore.js";

const builtins = [
  {
    id: "CreateNew",
    label: "Create new",
    component: defineAsyncComponent(() => import("./CreateNew/CreateNew.vue")),
    showInCreatePicker: false,
  },
  {
    id: "Editor",
    label: "Editor",
    component: Editor,
    settingComponent: CreateNewSetting,
  },
  {
    id: "IFrame",
    label: "IFrame",
    component: BorderlessIFrame,
    settingComponent: IFrameSetting,
    applyCreate(parentVm, settingFormRef) {
      if (settingFormRef && settingFormRef.src != null) {
        parentVm.$attrs.src = settingFormRef.src;
      }
    },
    toolbar: {
      showOpenInNewTab: true,
      showReload: true,
    },
  },
  {
    id: "Shortcut",
    label: "Shortcut",
    component: Shortcut,
    settingComponent: ShortcutSetting,
    applyCreate(parentVm, settingFormRef) {
      if (!settingFormRef) return;
      if (settingFormRef.link != null) parentVm.$attrs.link = settingFormRef.link;
      if (settingFormRef.img_link != null) {
        parentVm.$attrs.img_link = settingFormRef.img_link;
      }
      if (settingFormRef.title != null) {
        parentVm.$attrs.title = settingFormRef.title;
      }
      if (settingFormRef.description != null) {
        parentVm.$attrs.description = settingFormRef.description;
      }
    },
  },
  {
    id: "DockerManager",
    label: "DockerManager",
    component: DockerManager,
    settingComponent: DockerManagerSetting,
  },
  {
    id: "SystemMonitor",
    label: "Host metrics (all)",
    component: SystemMonitor,
    settingComponent: CreateNewSetting,
  },
  {
    id: "CpuMetrics",
    label: "CPU & load",
    component: CpuMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "MemoryMetrics",
    label: "Memory & swap",
    component: MemoryMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "DiskMetrics",
    label: "Disks",
    component: DiskMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "NetworkMetrics",
    label: "Network",
    component: NetworkMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "GpuMetrics",
    label: "GPU (NVIDIA)",
    component: GpuMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Terminal",
    label: "Terminal",
    component: Terminal,
    settingComponent: TerminalSetting,
  },
  {
    id: "CaddyConfigurator",
    label: "CaddyConfigurator",
    component: CaddyConfigurator,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Assistant",
    label: "AI assistant",
    component: Assistant,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Canvas",
    label: "Canvas",
    component: Canvas,
    settingComponent: CanvasSetting,
    applyCreate(parentVm, settingFormRef) {
      if (settingFormRef && settingFormRef.canvasMode != null) {
        parentVm.$attrs.canvasMode = settingFormRef.canvasMode;
      }
    },
  },
];

builtins.forEach((d) => registerApplet(d));
