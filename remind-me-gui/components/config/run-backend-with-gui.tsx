import { useState } from 'react';
import { Card } from '../ui/card';
import { Switch } from '../ui/switch';
import * as commands from '@/src/bindings';

export default function RunBackendWithGui(props: {
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
					<h1 className='font-bold'>Run backend with GUI</h1>
					<p className='max-w-[80%]'>
						When enabled, an instance of the backend will start when
						the GUI is launched. Disable this if you would like to
						separately manage the backend and GUI on your own.
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
