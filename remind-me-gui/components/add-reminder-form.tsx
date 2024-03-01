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
import EmojiPicker, { Emoji } from 'emoji-picker-react';
import { Dispatch, SetStateAction, useState } from 'react';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from './ui/dropdown-menu';
import { FaceIcon } from '@radix-ui/react-icons';

const formSchema = z.object({
	name: z.string(),
	description: z.string(),
	frequency: z
		.object({
			hours: z.coerce.number().optional(),
			minutes: z.coerce.number().optional(),
			seconds: z.coerce.number().optional(),
		})
		.refine(
			(data) => {
				// Check if at least one of the frequency fields is defined
				return (
					data.hours !== undefined ||
					data.minutes !== undefined ||
					data.seconds !== undefined
				);
			},
			{
				message:
					'At least one of the frequency fields (hours, minutes, seconds) must be defined',
			}
		),
	icon: z
		.string()
		.emoji({ message: 'This must be an emoji' })
		.max(2, {
			message: 'You may only set 1 emoji.',
		})
		.optional(),
});

export default function AddReminderForm(props: {
	path: string;
	setOpen: Dispatch<SetStateAction<boolean>>;
}) {
	const { path, setOpen } = props;
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
	});
	function onSubmit(values: z.infer<typeof formSchema>) {
		console.log('Adding a new reminder...', values);

		const freq =
			values.frequency.hours! * 60 * 60 +
			values.frequency.minutes! * 60 +
			values.frequency.seconds!;
		commands.addReminder(path, {
			name: values.name,
			description: values.description,
			frequency: freq,
			icon: values.icon as string | null,
		});
		setOpen(false);
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
					render={() => (
						<FormItem>
							<FormLabel>Frequency</FormLabel>
							<div className='w-full flex gap-5'>
								<FormField
									control={form.control}
									name='frequency.hours'
									render={({ field }) => (
										<FormItem>
											<FormControl>
												<Input
													placeholder='Hours...'
													{...field}
													type='number'
												/>
											</FormControl>

											<FormMessage />
										</FormItem>
									)}
								/>
								<FormField
									control={form.control}
									name='frequency.minutes'
									render={({ field }) => (
										<FormItem>
											<FormControl>
												<Input
													placeholder='Minutes...'
													{...field}
													type='number'
												/>
											</FormControl>

											<FormMessage />
										</FormItem>
									)}
								/>
								<FormField
									control={form.control}
									name='frequency.seconds'
									render={({ field }) => (
										<FormItem>
											<FormControl>
												<Input
													placeholder='Seconds...'
													{...field}
													type='number'
												/>
											</FormControl>

											<FormMessage />
										</FormItem>
									)}
								/>
							</div>
							<FormDescription>
								Sets how often this reminder will occur.
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
								<div className='relative flex items-center gap-1'>
									<Input placeholder='Icon...' {...field} />
									<DropdownMenu>
										<DropdownMenuTrigger asChild>
											<Button
												size='icon'
												variant='outline'
											>
												<FaceIcon />
											</Button>
										</DropdownMenuTrigger>
										<DropdownMenuContent>
											<DropdownMenuItem
												onSelect={(e) =>
													e.preventDefault()
												}
											>
												<EmojiPicker
													onEmojiClick={(e) =>
														form.setValue(
															'icon',
															e.emoji
														)
													}
												/>
											</DropdownMenuItem>
										</DropdownMenuContent>
									</DropdownMenu>
								</div>
							</FormControl>
							<FormDescription>
								Optionally, add an icon that will appear with
								the reminder! (note: as of now, the emoji picker
								is rather laggy, so be patient with it)
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>

				<div className='flex w-full justify-between'>
					<div>
						<Button type='submit'>Add</Button>
					</div>
				</div>
			</form>
		</Form>
	);
}
