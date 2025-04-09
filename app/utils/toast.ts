import { h } from 'vue';
import { useToast } from '~/components/ui/toast/use-toast';
import { ToastAction } from '~/components/ui/toast';

type ToastType = 'info' | 'warn' | 'error';

export function showToast(
  type: ToastType,
  message: string,
  options?: {
    title?: string;
    actionText?: string;
    onAction?: () => void;
  }
) {
  const { toast } = useToast();

  const variant = (
    {
      info: 'default' as const,
      warn: 'default' as const,
      error: 'destructive' as const,
    } as const
  )[type];

  return toast({
    title: options?.title || type.charAt(0).toUpperCase() + type.slice(1),
    description: message,
    variant,
    ...(options?.actionText
      ? {
          action: h(
            ToastAction,
            {
              altText: options.actionText,
              onClick: options.onAction || (() => window.location.reload()),
            },
            () => options.actionText
          ),
        }
      : {}),
  });
}

export function showErrorToast(error: unknown) {
  let message = 'An unknown error occurred';
  if (error instanceof Error) {
    message = error.message;
  } else if (typeof error === 'string') {
    message = error;
  }

  showToast('error', message, {
    title: 'Error',
    actionText: 'Refresh',
    onAction: () => window.location.reload(),
  });

  return new Error(message);
}
