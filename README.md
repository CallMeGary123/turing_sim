<h1 style="text-align: center;">Turing Machine Simulator</h1>
<p>This program aims to simulate behaviors of a Turing machine using <a href=https://en.wikipedia.org/wiki/Turing_machine>the formal definition</a>.</p>

<h2>Content</h2>
<ul>
<li><a href = #features>Features</a>
<ul>
<li><a href = #multi-track>Multi-track support</a></li>
<li><a href = #csv>CSV suppourt</a></li>
</ul></li>
<li><a href = #usage>Usage</a><ul>
<li><a href = #terminal>Define in terminal</a></li>
<li><a href = #from-csv>Load from CSV</a></li>
<li><a href = #demos>Demos</a></li>
</ul></li>
<li><a href=#args>Command-line arguments</a></li>
<li><a href=#issues>Issues</a></li>
</ul>

<h2 id = "features">Features</h2>
<h3 id = "multi-track">Multi-track support</h3>
<p>Although sometimes overlooked the formal definition of a Turing machine allows the usage of multiple tracks on a single tape. you can test this feature by loading demo 3.
<img src= "screenshots\1.png"></p>
<h3 id = "csv">CSV suppourt</h3>
<p>In order to give the program more of a sandbox feeling and allow for quick edits I've added CSV suppout which allows you to define transition functions in a CSV and load it into the program. example.csv defines a machine to add two numbers represented by 1s and seperated with a 0; With one track, initial state = q0 and final state = q4
<img src= "screenshots\2.png">
<b>*Note: </b> CSV file's headers must be <code>lhs_state,input,rhs_state,replacement,direction</code>.</p>

<h2 id = "usage">Usage</h2>
<h3 id = "terminal">Define in terminal</h3>
<p>You define all of your Turing maching by juts typing it into the terminal. to do this run the program without any command line arguments. <code>cargo run</code> or <code>&ltpath_to_compiled_program&gt</code> if you've compiled the program <img src= "screenshots\3.png"></p>
<h3 id = "from-csv">Load from CSV</h3>
<p>you can load a CSV by running <code>cargo run -- -csv &ltpath_to_your_file.csv&gt</code> or <code>&ltpath_to_compiled_program_name&gt -csv &ltpath_to_your_file.csv&gt</code> if you've compiled the program.<img src= "screenshots\2.png"><br>
<b>*Note: </b> CSV file's headers must be <code>lhs_state,input,rhs_state,replacement,direction</code>.</p>
</p>

<h3 id = "demos">Demos</h3>
<p>Run <code>cargo run -- -demo &ltdemo_number&gt</code> or <code>&ltpath_to_compiled_program_name&gt -demo &ltdemo_number&gt</code> if you've compiled the program.<br> You can find a list of all exisiting demos by runuing the program with the <code>-help</code> argument.</p>

<h3 id = "args">Command-line arguments</h3>
<p>Here's a list of program's command-line arguments.</p>
<table>
  <tr>
    <th>Arg</th>
    <th>Description</th>
  </tr>
  <tr>
    <td>-help</td>
    <td>Show the help menu.</td>
  </tr>
  <tr>
    <td>-demo &ltdemo_number&gt</td>
    <td>Load the specified demo.</td>
  </tr>
  <tr>
    <td>-csv</td>
    <td>Load trantition functions from the CSV file</td>
  </tr>
</table>


<h2 id = "issues">Issues</h2>
<p>Please refer to issues tab to see a list of current identified issues or submit your own issues and imporvment ideas.</p>