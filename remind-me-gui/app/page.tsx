'use client';
import Image from 'next/image';
import { Button } from '@/components/ui/button';
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from '@/components/ui/card';
import { Switch } from '@/components/ui/switch';
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import * as commands from '@/src/bindings';
import { Reminder } from '@/src/bindings';
import { watch } from 'tauri-plugin-fs-watch-api';
export default function Home() {
	const [reminders, setReminders] = useState<Reminder[]>();
	useEffect(() => {
		commands.readAllReminders('/home/kaya/.local/share/remind-me/Config.toml').then(res => {
			setReminders(res)
			console.log(res);
		}).catch(e => console.log(e));		

		const stopWatching = watch(
			"/home/kaya/.local/share/remind-me/Config.toml",
			(event) => {
				commands.readAllReminders('/home/kaya/.local/share/remind-me/Config.toml').then(res => {
					setReminders(res)
					console.log(res);
				}).catch(e => console.log(e));		
			},
			{ recursive: false },
		);

	}, []);


	const cards = reminders ? reminders.map((reminder, index) => {
		const minutes = Math.floor(reminder.frequency / 60);
		const seconds = reminder.frequency % 60;

		const handleEdit = () => {
			console.log('yello!');
		};
		return (
			<Card key={index} className='my-5'>
				<CardHeader>
					<div className='flex justify-between'>
						<div>
							<CardTitle>{reminder.name}</CardTitle>
							<CardDescription>
								{reminder.description}
							</CardDescription>
						</div>

						<Button variant='default' onClick={() => handleEdit()}>
							Edit
						</Button>
					</div>
				</CardHeader>
				<CardContent>
					<div>
						<h1 className='text-xl font-bold'>Frequency</h1>
						<p>
							Every
							{minutes > 0 && ` ${minutes} minutes`}
							{minutes > 0 && seconds > 0 && ','}
							{seconds > 0 && ` ${seconds} seconds`}
							.
						</p>
					</div>
				</CardContent>
			</Card>
		);
	}) : <p>loading</p>;

	return (
		<main className='mx-[10vw]'>
			<div className='my-5 flex justify-between items-start'>
				<div>
					<h1 className='text-3xl font-semibold'>Remind-me</h1>
					{/* TODO: randomize subheader! would be a fun detail :) */}
					<h2 className='mb-5 text-xl'>Welcome back!</h2>
					{/* <p>Your current reminders:</p> */}
					<Button variant='default'>New Reminder</Button>
				</div>
				<div className='flex gap-3'>
					<p>Auto-start</p>
					<Switch></Switch>
				</div>
			</div>
			<div>{cards}</div>
		</main>
	);
}
