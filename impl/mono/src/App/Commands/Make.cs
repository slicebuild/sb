using System.Collections.Generic;
using System.IO;
using System.Text;
using sb.Formatters;
using sb.Layers;
using sb.Slices;
using sb.Utils;

namespace sb.App.Commands
{
    public class Make : Find, ICommand
    {
        public Make(Args args) 
            : base(args)
        {
        }

        void ICommand.Run()
        {
            var missingNamesList = new List<string>();
            var layers = FindLayers(missingNamesList);
            var path = MakePath(layers[0]);
            if (missingNamesList.Count != 0)
            {
                ReportMissingRequested(layers, missingNamesList);
                return;                
            }

            Write(layers[0], path);
        }        

        /// <summary>
        /// Returns the path where to write the output.
        /// If option -o has been provided then this gets returned,
        /// otherwise it's the env root plus the layer's semname.
        /// If there is a file already at the path - it get's deleted.
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

            if (File.Exists(path))
                File.Delete(path);

            return path;
        }

        /// <summary>
        /// Writes the layer to disk using a formatter to prepare the output.
        /// </summary>
        /// <param name="layer"></param>
        /// <param name="path"></param>
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