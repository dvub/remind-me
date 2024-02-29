import { Switch } from '@/components/ui/switch';
import { Dialog, DialogTrigger, DialogContent } from '@/components/ui/dialog';
import { DialogHeader } from '../ui/dialog';
import { enable, isEnabled, disable } from 'tauri-plugin-autostart-api';
import { useEffect, useState } from 'react';
import Autostart from './autostart';
import { GearIcon } from '@radix-ui/react-icons';
import { Button } from '../ui/button';
import * as commands from '@/src/bindings';
import StartMinimized from './start-minimized';
import RunBackendWithGui from './run-backend-with-gui';
import ModeToggle from './theme-selector';
export default function Config() {
	let [path, setPath] = useState<string>();
	let [config, setConfig] = useState<commands.Config>();
	useEffect(() => {
		commands.getConfigPath().then((path) => {
			setPath(path);
			commands.readConfig(path).then((conf) => {
				setConfig(conf);
			});
		});
	}, []);
	return (
		<div>
			<Dialog>
				<DialogTrigger asChild>
					<Button size='icon' variant='default'>
						<GearIcon />
					</Button>
				</DialogTrigger>

				<DialogContent className=' overflow-y-scroll max-h-[90%]'>
					<DialogHeader>
						<h1 className='h1 text-xl font-bold'>Settings</h1>
					</DialogHeader>
					<div>
						<ModeToggle />
						<Autostart />
						<StartMinimized
							enabled={config?.start_minimized!}
							path={path!}
						/>

						<h1 className='font-bold text-red-600'>Advanced</h1>
						<p>
							Please proceed with caution. Modifying the following
							settings may cause the program to not function
							properly.
						</p>
						<RunBackendWithGui
							path={path!}
							enabled={config?.run_backend_on_gui_start!}
						/>
					</div>
				</DialogContent>
			</Dialog>
		</div>
	);
}
