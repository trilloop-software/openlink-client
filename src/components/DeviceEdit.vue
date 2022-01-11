<template>
  <q-dialog v-model="showDialog" persistent class="disable-select">
    <q-card>
      <q-bar>
        <q-item-label class="text-weight-bold">{{ newDevice ? 'Add Device' : 'Modify Device' }}</q-item-label>
        <q-space />
        <q-btn flat icon="close" v-close-popup />
      </q-bar>

      <q-card-section class="column justify-center items-center">
        <q-icon color="primary" :name="getDeviceIcon(activeDevice.device_type)" size="4rem" />
        <q-item-label caption class="text-uppercase">{{ activeDevice.device_type }}</q-item-label>
      </q-card-section>
      <q-card-section class="q-gutter-y-md">
        <q-input
          dense
          filled
          v-model="activeDevice.name"
          label="Name"
        />
        <q-input
          dense
          filled
          v-model="activeDevice.ip_address"
          label="IP Address"
        />
        <q-input
          dense
          filled
          v-model.number="activeDevice.port"
          label="Port"
          :rules="[val => (val >= 0 && val <= 65535) || 'Invalid port range.']"
        />
      </q-card-section>
      <q-card-actions class="fit row justify-between">
        <q-btn v-if="!newDevice"
          color="red"
          flat
          dense
          icon="delete"
          label="REMOVE"
          @click="removeDevice(activeDevice)"
        />

        <q-btn
          color="primary"
          flat
          dense
          icon="settings_ethernet"
          label="PING"
          @click="activeDevice.ping(activeDevice.name, activeDevice.ip_address, activeDevice.port)"
        />

        <q-btn v-if="newDevice"
          color="primary"
          flat
          dense
          icon-right="add"
          label="ADD"
          @click="addDevice(activeDevice)"
        />

        <q-btn v-else
          color="primary"
          flat
          dense
          icon-right="arrow_upward"
          label="UPDATE"
          @click="modifyDevice(activeDevice)"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { PropType } from 'vue'
import { Device, getDeviceIcon } from '@/libs/device'
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'DeviceEdit',
  props: {
    new: { type: Boolean, default: false },
    device: { type: Object as PropType<Device> },
    show: { type: Boolean, default: false }
  },
  emits: ['update:show','update:device','update:new','add-device','modify-device','remove-device'],
  setup: (props: any, { emit }) => {
    // emit new device to parent component to add/modify/remove to/from device list
    // need to implement in backend / rust frontend
    function addDevice(dev: Device) {
      emit('add-device', dev)
    }

    function modifyDevice(dev: Device) {
      emit('modify-device', dev)
    }

    function removeDevice(dev: Device) {
      emit('remove-device', dev)
    }

    return {
      getDeviceIcon,
      addDevice,
      modifyDevice,
      removeDevice,
      showDialog: useModelWrapper(props, emit, 'show'),
      newDevice: useModelWrapper(props, emit, 'new'),
      activeDevice: useModelWrapper(props, emit, 'device'),
    }
  }
}
</script>