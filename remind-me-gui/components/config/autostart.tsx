import { useState, useEffect } from 'react';
import { enable, disable, isEnabled } from 'tauri-plugin-autostart-api';
import { Switch } from '../ui/switch';

export default function Autostart() {
	function handleAutoStartChange(e: boolean) {
		console.log('new val:', e);
		setAutostart(e);
		if (e) {
			enable();
		} else {
			disable();
		}
	}
	const [autostart, setAutostart] = useState<boolean>();
	useEffect(() => {
		async function s() {
			const enabled = await isEnabled();
			setAutostart(enabled);
		}
		s();
	}, []);
	return (
		<div className='flex w-full justify-between my-5'>
			<div>
				<h1 className='font-bold'>Auto-start</h1>
				<p className='max-w-[80%]'>
					Enabling this will start the program when you log in to your
					computer.
				</p>
			</div>
			<Switch
				checked={autostart}
				onCheckedChange={(e) => handleAutoStartChange(e)}
			/>
		</div>
	);
}
