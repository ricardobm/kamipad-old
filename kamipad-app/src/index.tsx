import * as React from 'react'
import * as ReactDOM from 'react-dom'

import App from './App'

import './css/main.scss'

console.log("Hello world from Kamipad!")
ReactDOM.render(
	<React.StrictMode>
		<App />
	</React.StrictMode>,
	document.querySelector('#app'),
)
