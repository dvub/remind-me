import { Reminder } from '@/src/bindings';
import EditReminderForm from './edit-reminder-form';
import { Button } from '../ui/button';
import {
	DialogHeader,
	Dialog,
	DialogTrigger,
	DialogContent,
	DialogDescription,
} from '../ui/dialog';
import { Pencil1Icon } from '@radix-ui/react-icons';
import { DropdownMenuItem } from '../ui/dropdown-menu';

export default function EditReminderDialog(props: {
	reminder: Reminder;
	path: string;
}) {
	const { reminder, path } = props;
	return (
		<Dialog>
			<DialogTrigger asChild>
				<DropdownMenuItem onSelect={(e) => e.preventDefault()}>
					<div className='flex gap-1 items-center'>
						<Pencil1Icon />
						Edit
					</div>
				</DropdownMenuItem>
			</DialogTrigger>
			<DialogContent className=' overflow-y-scroll max-h-[90%]'>
				<DialogHeader>
					<h1 className='h1 text-xl font-bold'>Edit Reminder</h1>
				</DialogHeader>
				<DialogDescription>
					Edit the current reminder. All inputs are optional; leaving
					an input blank will not update that data on the current
					reminder.
				</DialogDescription>

				<EditReminderForm name={reminder.name} path={path} />
			</DialogContent>
		</Dialog>
	);
}
