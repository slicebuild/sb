using System.Collections.Generic;
using System.IO;
using System.Text;
using sb.Formatters;
using sb.Formatters.Docker;
using sb.Layers;
using sb.Slices;
using sb.Utils;

namespace sb.App.Commands
{
    public class Make : ICommand
    {
        public Make(Args args)
        {
            Args = args;
        }

        public Args Args { get; }

        void ICommand.Run()
        {
            var layer = MakeLayer();
            var path = MakePath(layer);
            Write(layer, path);
        }

        public virtual Layer MakeLayer()
        {
            var layerParams = Args.GetLayerParams();
            var osParam = Args.GetOsParam();

            var list = new List<Slice>();

            var dirList = new SliceDirectoryList(Args.SlicesDir);
            dirList.ForEach(dir => list.AddRange(dir.FindByOs(osParam)));

            var layers = new LayerList(list);
            var layer = layers.FindLayer(layerParams[0]);

            // if there were more layers requested, add them as dependencies
            for (var i = 1; i < layerParams.Length; i++)
            {
                var additionalLayer = layers.FindLayer(layerParams[i]);
                layer.Dependencies.Insert(i - 1, additionalLayer);
            }
            return layer;
        }

        /// <summary>
        /// Returns the path where to write the output.
        /// If option -o has been provided then this gets returned,
        /// otherwise it's the env root plus the layer's semname
        /// </summary>
        /// <param name="layer"></param>
        /// <returns>path where to save the output</returns>
        public virtual string MakePath(Layer layer)
        {
            var path = Args.GetOutPath();
            if (path.IsEmpty())
                path = Path.Combine(Args.EnvDir, layer.SemVerName.ToString());
            var dir = Path.GetDirectoryName(path);
            if (dir != null && !Directory.Exists(dir))
                Directory.CreateDirectory(dir);
            return path;
        }

        public virtual void Write(Layer layer, string path)
        {
            var formatter = new FormatterFactory().GetFormatter(Args);
            Write(layer, path, formatter);
        }

        public virtual void Write(Layer layer, string path, IFormatter formatter)
        {
            var lines = new List<string>();
            Write(layer, lines);

            var slice = new Slice("", layer.SemVerName, lines);

            var sb = new StringBuilder();
            formatter.Write(slice, sb);

            using (var fs = File.OpenWrite(path))
            using (var swr = new StreamWriter(fs))
            {
                swr.WriteLine(sb.ToString());
                swr.Flush();
            }
        }

        public virtual void Write(Layer layer, List<string> lines)
        {
            foreach (var dep in layer.Dependencies)
            {
                Write(dep, lines);
                dep.Written = true;
            }

            if (!layer.Written)
            {
                foreach (var section in layer.Sections)
                {
                    lines.Add(section.SectionType.ToString());
                    lines.AddRange(section.Lines);
                }
            }
        }        
    }
}