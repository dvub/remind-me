import { Switch } from '@/components/ui/switch';

export default function Config() {
	return (
		<div className='flex gap-3'>
			<p>Auto-start</p>
			<Switch></Switch>
		</div>
	);
}
