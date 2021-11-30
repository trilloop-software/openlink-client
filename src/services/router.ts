import { createRouter, createWebHashHistory } from 'vue-router'

import Configure from '@/pages/Configure.vue'
import Dashboard from '@/pages/Dashboard.vue'
import Login from '@/pages/Login.vue'
import Manage from '@/pages/Manage.vue'

export default createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            name: 'Configure',
            path: '/configure',
            component: Configure
        },
        {
            name: 'Dashboard',
            path: '/dashboard',
            component: Dashboard
        },
        {
            name: 'Login',
            path: '/login',
            component: Login
        },
        {
            name: 'Manage',
            path: '/manage',
            component: Manage
        },
        {
            path: '/.*',
            redirect: '/login',
        },
        {
            path: '/',
            redirect: '/login',
        },
    ]
})