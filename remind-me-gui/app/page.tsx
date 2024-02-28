'use client';
import { Button } from '@/components/ui/button';
import { useEffect, useState } from 'react';
import * as commands from '@/src/bindings';
import { Reminder } from '@/src/bindings';

import ReminderCard from '@/components/reminder/reminder-card';
import Config from '@/components/config/config';
import AddReminderDialog from '@/components/add-reminder-dialog';


// TODO: 
// double check and fix any state/effect issues
// clean up code

// UI/UX
// add DARK MODE!!! 
// improve looks
// fix add/edit forms PLEASE
// - add/save buttons close the form
// 

// FEATURES
// add priority/timeout for reminders
// add limits for reminders (i.e. reminders can appear once, twice, or forever)

// BUGS:
// cpu usage issue - FIXED
// reminders are sent by powershell on windows

// MISC
// improve Releases page on the tauri action
// maybe do a custom window border?
// change window name, process name

// DISTRIBUTION:
// read and implement everything on tauri distribution guide

// GITHUB
// improve github page!
// readme
// move todos to github
// that kind of thing

// WEBSITE
// start building website

export default function Home() {
	// console.log("Is autostarting?", isEnabled());
	// enable();

	const [path, setPath] = useState<string>('');
	const [reminders, setReminders] = useState<Reminder[]>();

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

		async function setUpWatching() {
			return await (
				await import('tauri-plugin-fs-watch-api')
			).watch(
				path,
				(e) => {
					updateReminders(path);
				},
				{ delayMs: 0, recursive: false }
			);
		}
		setUpWatching();
	}, [path]);

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
				<div className='w-full'>
					<h1 className='text-3xl font-semibold'>Remind-me</h1>
					{/* TODO: randomize subheader! would be a fun detail :) */}
					<h2 className='mb-5 text-xl'>Welcome back!</h2>
					{/* <p>Your current reminders:</p> */}

					<AddReminderDialog path={path} />
				</div>
				<Config />
			</div>
			{reminders && (
				<div>
					{reminders.length > 0 && <div>{cards}</div>}
					{reminders.length === 0 && (
						<div className='w-full flex justify-center text-center text-black/50'>
							<p>No reminders found... </p>
						</div>
					)}
				</div>
			)}
			{!reminders && <p>Loading</p>}
		</main>
	);
}
