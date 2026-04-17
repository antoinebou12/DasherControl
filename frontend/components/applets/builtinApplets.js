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
    category: "content",
    component: Editor,
    settingComponent: CreateNewSetting,
  },
  {
    id: "IFrame",
    label: "IFrame",
    category: "content",
    component: BorderlessIFrame,
    settingComponent: IFrameSetting,
    applyCreate(parentVm, settingFormRef) {
      if (settingFormRef && settingFormRef.src != null) {
        parentVm.applyCreateAttrs({ src: settingFormRef.src });
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
    category: "content",
    component: Shortcut,
    settingComponent: ShortcutSetting,
    applyCreate(parentVm, settingFormRef) {
      if (!settingFormRef) return;
      const attrs = {};
      if (settingFormRef.link != null) attrs.link = settingFormRef.link;
      if (settingFormRef.img_link != null) attrs.img_link = settingFormRef.img_link;
      if (settingFormRef.title != null) attrs.title = settingFormRef.title;
      if (settingFormRef.description != null) {
        attrs.description = settingFormRef.description;
      }
      parentVm.applyCreateAttrs(attrs);
    },
  },
  {
    id: "DockerManager",
    label: "DockerManager",
    category: "tools",
    component: DockerManager,
    settingComponent: DockerManagerSetting,
  },
  {
    id: "SystemMonitor",
    label: "Host metrics (all)",
    category: "metrics",
    component: SystemMonitor,
    settingComponent: CreateNewSetting,
  },
  {
    id: "CpuMetrics",
    label: "CPU & load",
    category: "metrics",
    component: CpuMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "MemoryMetrics",
    label: "Memory & swap",
    category: "metrics",
    component: MemoryMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "DiskMetrics",
    label: "Disks",
    category: "metrics",
    component: DiskMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "NetworkMetrics",
    label: "Network",
    category: "metrics",
    component: NetworkMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "GpuMetrics",
    label: "GPU (NVIDIA)",
    category: "metrics",
    component: GpuMetrics,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Terminal",
    label: "Terminal",
    category: "tools",
    component: Terminal,
    settingComponent: TerminalSetting,
  },
  {
    id: "CaddyConfigurator",
    label: "CaddyConfigurator",
    category: "tools",
    component: CaddyConfigurator,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Chat",
    label: "Chat",
    category: "ai",
    component: Assistant,
    settingComponent: CreateNewSetting,
  },
  {
    id: "Canvas",
    label: "Canvas",
    category: "content",
    component: Canvas,
    settingComponent: CanvasSetting,
    applyCreate(parentVm, settingFormRef) {
      if (settingFormRef && settingFormRef.canvasMode != null) {
        parentVm.applyCreateAttrs({ canvasMode: settingFormRef.canvasMode });
      }
    },
  },
];

builtins.forEach((d) => registerApplet(d));
