import { mount } from 'svelte';
import App from './App.svelte';
import Bench from './Bench.svelte';
import './styles.css';

const isBench = new URLSearchParams(location.search).has('bench');

mount(isBench ? Bench : App, { target: document.getElementById('app')! });
