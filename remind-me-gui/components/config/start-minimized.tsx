import { useState } from 'react';
import { Card } from '../ui/card';
import { Switch } from '../ui/switch';
import * as commands from '@/src/bindings';

export default function StartMinimized(props: {
	path: string;
	enabled: boolean;
}) {
	const { path, enabled } = props;
	const [isEnabled, setIsEnabled] = useState(enabled);

	const handleCheckedChange = (e: boolean) => {
		// enabled = e;
		console.log(e);
		commands.updateStartMinimized(path, e);
		setIsEnabled(e);
	};
	return (
		<Card className='m-3 px-5'>
			<div className='flex w-full justify-between my-5'>
				<div>
					<h1 className='font-bold'>Start minimized</h1>
					<p className='max-w-[80%]'>
						When enabled, the program will start minimized. (The
						program can be opened by opening the menu from the tray
						icon)
					</p>
				</div>
				<Switch
					checked={isEnabled}
					onCheckedChange={(e) => handleCheckedChange(e)}
				/>
			</div>
		</Card>
	);
}
