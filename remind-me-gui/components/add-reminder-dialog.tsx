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

export default function AddReminderDialog(props: { path: string }) {
	const { path } = props;
	return (
		<Dialog>
			<DialogTrigger asChild>
				<Button>
					{/*
				<div>
					<Button variant='default'>Add Reminder</Button>
				</div>
				*/}
					Add Reminder
				</Button>
			</DialogTrigger>
			<DialogContent className=' overflow-y-scroll max-h-[90%]'>
				<DialogHeader>
					<h1 className='h1 text-xl font-bold'>New Reminder</h1>
				</DialogHeader>
				{/*
				<DialogDescription>
					Edit the current reminder.
				</DialogDescription>
                */}
				<AddReminderForm path={path} />
			</DialogContent>
		</Dialog>
	);
}
