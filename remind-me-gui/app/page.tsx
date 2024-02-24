'use client';
import { Button } from '@/components/ui/button';
import { useEffect, useState } from 'react';
import * as commands from '@/src/bindings';
import { Reminder } from '@/src/bindings';
import { watch } from 'tauri-plugin-fs-watch-api';
import ReminderCard from '@/components/reminder-card';
import Config from '@/components/config';

export default function Home() {
	// console.log("Is autostarting?", isEnabled());
	// enable();

	const [path, setPath] = useState<string>('');
	const [reminders, setReminders] = useState<Reminder[]>();

	watch(
		path,
		(event) => {
			updateReminders(path);
		},
		// TODO: find the optimal ms
		{ recursive: false, delayMs: 100 }
	);

	const updateReminders = (path: string) => {
		commands
			.readAllReminders(path)
			.then((res) => {
				setReminders(res);
				console.log('Reminders:', res);
			})
			.catch((e) =>
				console.log('There was an error fetching reminders:', e)
			);
	};

	useEffect(() => {
		// INITIALIZE
		// set path state ASAP
		commands
			.getPath()
			.then((res) => {
				setPath(res);
				updateReminders(res);
				console.log('Current path:', res);
			})
			.catch((e) => console.log('There was an error getting the path!'));
		// TODO:
		// needs return here?
	}, []);

	const cards = reminders ? (
		reminders.map((reminder, index) => {
			console.log(reminder);
			return <ReminderCard reminder={reminder} key={index} path={path} />;
		})
	) : (
		<p>loading</p>
	);

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
				<Config />
			</div>
			{reminders && (
				<div>
					{reminders.length > 0 && <div>{cards}</div>}
					{reminders.length === 0 && (
						<div className='w-full text-center text-black/50'>
							<p>No reminders found... </p>
						</div>
					)}
				</div>
			)}
			{!reminders && <p>Loading</p>}
		</main>
	);
}
