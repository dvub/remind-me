'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from '@/components/ui/form';
import { DialogClose } from './ui/dialog';
import { Input } from '@/components/ui/input';
import * as commands from '@/src/bindings';

const formSchema = z.object({
	name: z.string(),
	description: z.string(),
	frequency: z.coerce.number().finite().positive().int().safe(),
	icon: z
		.string()
		.emoji({ message: 'This must be an emoji' })
		.max(3, {
			message:
				'You may only enter a maximum of 3 emojis! (3 is already a lot in my opinion)',
		})
		.optional(),
});

export default function AddReminderForm(props: { path: string }) {
	const { path } = props;
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
	});
	function onSubmit(values: z.infer<typeof formSchema>) {
		console.log('Adding a new reminder...', values);
		commands.addReminder(path, values as commands.Reminder);
	}

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
				<FormField
					control={form.control}
					name='name'
					render={({ field }) => (
						<FormItem>
							<FormLabel>Name</FormLabel>
							<FormControl>
								<Input placeholder='Name...' {...field} />
							</FormControl>
							<FormDescription>
								This will be the name of your reminder. Try to
								keep it short and memorable!
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name='description'
					render={({ field }) => (
						<FormItem>
							<FormLabel>Description</FormLabel>
							<FormControl>
								<Input
									placeholder='Description...'
									{...field}
								/>
							</FormControl>
							<FormDescription>
								Provide some extra information, details, or
								notes about your reminder!
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name='frequency'
					render={({ field }) => (
						<FormItem>
							<FormLabel>Frequency</FormLabel>
							<FormControl>
								<Input placeholder='Frequency...' {...field} />
							</FormControl>
							<FormDescription>
								How often do you want this reminder to appear?
								Currently, only entering values in seconds is
								supported. (i.e. 600 seconds = 10 minutes)
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name='icon'
					render={({ field }) => (
						<FormItem>
							<FormLabel>Icon</FormLabel>
							<FormControl>
								<Input placeholder='Icon...' {...field} />
							</FormControl>
							<FormDescription>
								Optionally, add up to 3 emojis that will appear
								with the reminder! (Currently there is no simple
								way to input emojis, good luck)
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>
				<div className='flex w-full justify-between'>
					<div>
						<Button type='submit'>Add</Button>
					</div>
					<DialogClose>
						<Button variant='default' type='button'>
							Close
						</Button>
					</DialogClose>
				</div>
			</form>
		</Form>
	);
}
