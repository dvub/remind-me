import { Reminder } from '@/src/bindings';
import EditReminderForm from './edit-reminder-form';
import { Button } from './ui/button';
import {
	DialogHeader,
	Dialog,
	DialogTrigger,
	DialogContent,
	DialogDescription,
} from './ui/dialog';

export default function EditReminderDialog(props: {reminder: Reminder, path: string}) {
    const { reminder, path } = props;
	return (
		<Dialog>
			<DialogTrigger>
				<Button variant='default'>Edit</Button>
			</DialogTrigger>
			<DialogContent>
				<DialogHeader>
					<h1 className='h1 text-xl font-bold'>Edit Reminder</h1>
				</DialogHeader>
				{/*
				<DialogDescription>
					Edit the current reminder.
				</DialogDescription>
                */}
				<EditReminderForm name={reminder.name} path={path}/>
			</DialogContent>
		</Dialog>
	);
}
