using System.Collections.Generic;
using System.IO;
using System.Text;
using sb.Core.Formatters;
using sb.Core.Layers;
using sb.Core.Slices;
using sb.Core.Utils;

namespace sb.Core.App.Commands
{
    public class Make : Find, ICommand
    {
        public Make(Args args) 
            : base(args)
        {
        }

        void ICommand.Run()
        {
            var layers = BuildLayers();
            var path = MakePath(layers[0], Args.MakeDir);
            if (layers.MissingInfos.Count != 0)
            {
                ReportMissingRequested(layers);
                return;                
            }

            Write(path, layers.OsLayer, layers[0]);
        }

        /// <summary>
        /// Returns the path where to write the output.
        /// If option -o has been provided then this gets returned,
        /// otherwise it's the env root plus the layer's semname.
        /// If there is a file already at the path - it get's deleted.
        /// </summary>
        /// <param name="layer"></param>
        /// <param name="outPath"></param>
        /// <returns>path where to save the output</returns>
        public virtual string MakePath(Layer layer, string outPath)
        {
            var path = Args.GetOutPath();
            if (path.IsEmpty())
                path = Path.Combine(outPath, layer.Slice.Info.ToString());

            var dir = Path.GetDirectoryName(path);
            if (dir != null && !Directory.Exists(dir))
                Directory.CreateDirectory(dir);

            if (File.Exists(path))
                File.Delete(path);

            return path;
        }

        /// <summary>
        /// Writes the layer to disk using a formatter to filter the output.
        /// </summary>
        /// <param name="layer"></param>
        /// <param name="path"></param>
        public virtual void Write(string path, Layer osLayer, Layer layer)
        {
            var formatter = new FormatterFactory().GetFormatter(Args);
            Write(path, osLayer, layer, formatter);
        }

        public virtual void Write(string path, Layer osLayer, Layer layer, IFormatter formatter)
        {
            var lines = new List<string>();

            osLayer.Write(lines);
            layer.Write(lines);

            var slice = new Slice("", layer.Slice.Info, lines);

            var sb = new StringBuilder();
            formatter.Write(slice, sb);

            using (var fs = File.OpenWrite(path))
            using (var swr = new StreamWriter(fs))
            {
                swr.WriteLine(sb.ToString());
                swr.Flush();
            }
        }        
    }
}
