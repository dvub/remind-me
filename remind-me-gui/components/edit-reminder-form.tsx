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
const formSchema = z
	.object({
		name: z.string().optional(),
		description: z.string().optional(),
		frequency: z.coerce
			.number()
			.finite()
			.positive()
			.int()
			.safe()
			.optional(),
		icon: z
			.string()
			.emoji({ message: 'This must be an emoji' })
			.max(3, {
				message:
					'You may only enter a maximum of 3 emojis! (3 is already a lot in my opinion)',
			})
			.optional(),
	})
	.refine((data) => Object.values(data).some((v) => v !== undefined), {
		message: 'You must make at least one change. ',
	});

export default function EditReminderForm(props: {
	path: string;
	name: string;
}) {
	const { path, name } = props;
	// 1. Define your form.
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		defaultValues: {
			name: undefined,
			description: undefined,
			icon: undefined,
			frequency: undefined,
		},
	});

	// 2. Define a submit handler.
	function onSubmit(values: z.infer<typeof formSchema>) {
		console.log('Submitting!!', values);
		commands.editReminder(path, name, values as commands.EditReminder);
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
							<FormMessage />
						</FormItem>
					)}
				/>
				<div className='flex w-full justify-between'>
					<Button type='submit'>Edit</Button>
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
