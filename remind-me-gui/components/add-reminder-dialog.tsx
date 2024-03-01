import { Reminder } from '@/src/bindings';
import EditReminderForm from './reminder/edit-reminder-form';
import { Button } from './ui/button';
import {
	DialogHeader,
	Dialog,
	DialogTrigger,
	DialogContent,
	DialogDescription,
} from './ui/dialog';
import { ScrollArea } from '@/components/ui/scroll-area';
import AddReminderForm from './add-reminder-form';
import { CardStackPlusIcon } from '@radix-ui/react-icons';
import { useState } from 'react';

export default function AddReminderDialog(props: { path: string }) {
	const { path } = props;
	const [open, setOpen] = useState(false);
	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger asChild>
				<Button size='icon'>
					<CardStackPlusIcon />
				</Button>
			</DialogTrigger>
			<DialogContent className=' overflow-y-scroll max-h-[90%]'>
				<DialogHeader>
					<h1 className='h1 text-xl font-bold'>New Reminder</h1>
				</DialogHeader>
				<AddReminderForm path={path} setOpen={setOpen} />
			</DialogContent>
		</Dialog>
	);
}
