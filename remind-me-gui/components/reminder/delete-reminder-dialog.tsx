import {
	AlertDialog,
	AlertDialogTrigger,
	AlertDialogContent,
	AlertDialogTitle,
	AlertDialogDescription,
	AlertDialogCancel,
	AlertDialogAction,
} from '@/components/ui/alert-dialog';
import { AlertDialogHeader, AlertDialogFooter } from '../ui/alert-dialog';
import { TrashIcon } from '@radix-ui/react-icons';
import { Button } from '../ui/button';
import { deleteReminder } from '@/src/bindings';
import { DropdownMenuItem } from '../ui/dropdown-menu';

export default function DeleteReminderDialog(props: {
	path: string;
	name: string;
}) {
	const { path, name } = props;
	return (
		<AlertDialog>
			<AlertDialogTrigger asChild>
				<DropdownMenuItem onSelect={(e) => e.preventDefault()}>
					<div className='flex gap-1 items-center'>
						<TrashIcon /> Delete
					</div>
				</DropdownMenuItem>
			</AlertDialogTrigger>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>
						Are you absolutely sure?
					</AlertDialogTitle>
					<AlertDialogDescription>
						This action cannot be undone. This reminder will be gone
						forever! (a very long time.)
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel>Cancel</AlertDialogCancel>
					<AlertDialogAction
						onClick={() =>
							deleteReminder(path, name).then((res) =>
								console.log('Successfully deleted:', res)
							)
						}
					>
						Delete
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
