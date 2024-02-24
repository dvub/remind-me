import { Reminder } from '@/src/bindings';
import EditReminderDialog from './edit-reminder-dialog';
import {
	Card,
	CardHeader,
	CardTitle,
	CardDescription,
	CardContent,
	CardFooter,
} from './ui/card';
import { TrashIcon } from '@radix-ui/react-icons';
import { Button } from './ui/button';
import { Trash } from 'lucide-react';
import DeleteReminderDialog from './delete-reminder-dialog';

export default function ReminderCard(props: {
	reminder: Reminder;
	path: string;
}) {
	const { reminder, path } = props;
	const minutes = Math.floor(reminder.frequency / 60);
	const seconds = reminder.frequency % 60;

	return (
		<Card className='my-5'>
			<CardHeader>
				<div className='flex justify-between'>
					<div>
						<CardTitle>{reminder.name}</CardTitle>
						<CardDescription>
							{reminder.description}
						</CardDescription>
					</div>
					<EditReminderDialog reminder={reminder} path={path} />
				</div>
			</CardHeader>
			<CardContent>
				<div>
					<h1 className='text-xl font-bold'>Frequency</h1>
					<p>
						Every
						{minutes > 0 && ` ${minutes} minutes`}
						{minutes > 0 && seconds > 0 && ','}
						{seconds > 0 && ` ${seconds} seconds`}.
					</p>
				</div>
			</CardContent>
			<CardFooter>
				<div className=''>
					<DeleteReminderDialog path={path} name={reminder.name} />
				</div>
			</CardFooter>
		</Card>
	);
}
