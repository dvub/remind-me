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
	name: z.string().optional(),
	description: z.string().optional(),
	frequency: z.number().optional(),
	icon: z.string().optional(),
});

export default function EditReminderForm(props: {
	path: string;
	name: string;
}) {
	const { path, name } = props;
	// 1. Define your form.
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
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
							<FormDescription>
								This will be the name of your reminder. Try to
								make it short!
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
								Optionally, add a symbol or emoji that will
								appear with the reminder!
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>
				<DialogClose>
					<Button type='submit'>Submit</Button>
				</DialogClose>
			</form>
		</Form>
	);
}
