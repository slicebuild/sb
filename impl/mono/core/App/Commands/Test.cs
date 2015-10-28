using System.Collections.Generic;
using System.IO;
using sb.Core.Utils;

namespace sb.Core.App.Commands
{
    public class Test : Make, ICommand
    {
        public Test(Args args)
            : base(args)
        {
        }

        void ICommand.Run()
        {
            Args.SetOption(Args.Options.Format, Args.OptionDefaults.FormatDocker);

            var layers = BuildLayers();
            var path = MakePath(layers[0], Args.TestDir);

            if (layers.MissingInfos.Count != 0)
            {
                ReportMissingRequested(layers);
                return;
            }

            Write(path, layers.OsLayer, layers[0]);

            var stdout = new List<string>();
            Fs.RunProcess("docker", $"build -t sb-test {Path.GetDirectoryName(path)}", stdout);

            if (stdout.Count > 0)
            {
                var items = stdout[stdout.Count - 1].Split(" ");
                if (items.Length >= 3 && items[0].StartsWith("Success"))
                {
                    //var id = items[2];
                    //todo: use the generated image id
                }
            }
        }
    }
}