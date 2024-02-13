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

export default function Home() {
	const reminders = [
		{
			title: 'Stretch',
			description: "Don't forget to stretch!",
			frequency: 128,
		},
		{
			title: 'Drink water',
			description: 'Remember to stay hydrated!',
			frequency: 600,
		},
	];

	const cards = reminders.map((reminder, index) => {
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
							<CardTitle>{reminder.title}</CardTitle>
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
							Every {minutes} minutes
							{seconds > 0 && `, ${seconds} seconds`}.
						</p>
					</div>
				</CardContent>
			</Card>
		);
	});

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
