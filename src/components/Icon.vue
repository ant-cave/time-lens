<script setup>
import {
  Key,
  Plus,
  Download,
  SwitchButton,
  View,
  Delete,
  Setting,
  Brush,
  Sunny,
  Moon,
  Monitor,
  Lock,
  Check,
  Warning,
  Close,
  Folder,
  Search,
  ArrowDown,
  Menu,
  User,
  Message,
  QuestionFilled
} from '@element-plus/icons-vue'

// 图标映射表
const ICON_COMPONENTS = {
  Key,
  Plus,
  Download,
  'sign-out': SwitchButton,
  View,
  Delete,
  Setting,
  Brush,
  Sunny,
  Moon,
  Monitor,
  Lock,
  Check,
  Warning,
  Close,
  Folder,
  Search,
  ArrowDown,
  Menu,
  User,
  Message,
  key: Key,
  plus: Plus,
  download: Download,
  eye: View,
  trash: Delete,
  gear: Setting,
  paintbrush: Brush,
  sun: Sunny,
  moon: Moon,
  desktop: Monitor,
  shield: Lock,
  check: Check,
  alert: Warning,
  x: Close,
  repo: Folder,
  'mark-github': Folder,
  'triangle-down': ArrowDown,
  'three-bars': Menu,
  person: User,
  account: User,
  mail: Message,
  key2: Key
}

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  size: {
    type: Number,
    default: 16
  }
})

const iconComponent = ICON_COMPONENTS[props.name] || QuestionFilled

const style = {
  width: `${props.size}px`,
  height: `${props.size}px`,
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center'
}
</script>

<template>
  <component :is="iconComponent" :style="style" />
</template>
