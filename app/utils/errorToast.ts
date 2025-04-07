import type { Updater } from '@tanstack/vue-table';
import type { Ref } from 'vue';
import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { h } from 'vue';
import { useToast } from '~/components/ui/toast/use-toast';
import { ToastAction } from '~/components/ui/toast';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function valueUpdater<T extends Updater<any>>(updaterOrValue: T, ref: Ref) {
  ref.value = typeof updaterOrValue === 'function' ? updaterOrValue(ref.value) : updaterOrValue;
}

export function showErrorToast(error: unknown) {
  const { toast } = useToast();

  let message = 'An unknown error occurred';
  if (error instanceof Error) {
    message = error.message;
  } else if (typeof error === 'string') {
    message = error;
  }

  toast({
    title: 'Error',
    description: message,
    variant: 'destructive',
    action: h(
      ToastAction,
      {
        altText: 'Refresh page',
        onClick: () => window.location.reload(),
      },
      {
        default: () => 'Refresh',
      }
    ),
  });

  return new Error(message);
}
