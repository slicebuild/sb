using sb.Core.App;
using sb.Core.Formatters.Docker;
using sb.Core.Formatters.Shell;

namespace sb.Core.Formatters
{
    public class FormatterFactory
    {
        public IFormatter GetFormatter(Args args) //todo: replace args with formatter option for better separation
        {
            //todo:check formatter found, throw
            var formatOption = args.GetOption(Args.Options.Format, Args.OptionDefaults.FormatShell);

            if (formatOption.StartsWith(Args.OptionDefaults.FormatDocker))
                return new FormatterDocker();
            if (formatOption.StartsWith(Args.OptionDefaults.FormatShell))
                return new FormatterShell();

            return null;
        }
    }
}