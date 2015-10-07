using cb.App;
using cb.Formatters.Docker;
using cb.Formatters.Shell;

namespace cb.Formatters
{
    public class FormatterFactory
    {
        public IFormatter GetFormatter(Args args) //todo: replace args with formatter option for better separation
        {
            //todo:check formatter found, throw
            var formatOption = args.GetOption(Args.Options.Format, Args.OptionValues.FormatShell);

            if (formatOption.StartsWith(Args.OptionValues.FormatDocker))
                return new FormatterDocker();
            if (formatOption.StartsWith(Args.OptionValues.FormatShell))
                return new FormatterShell();

            return null;
        }
    }
}