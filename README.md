<h2>Overview</h2>
A collection of simple projects I created to learn the basics of Rust, which make use of various data structures and function types along with producing audio and graphical outputs based on user inputs.<br/>
More projects may be added to this over time.

<h4>Guess the Number</h4>
This is the first and most simple of the projects I made. The user is prompted to enter a number, which is then compared to a randomly generated number and feedback is given until the user guesses correctly.

<h4>Web Scraper</h4>
This web scraper makes use of [Reqwest](https://crates.io/crates/reqwest) and [Scraper](https://crates.io/crates/scraper) to get and parse the HTML of a given webpage.<br/>
I designed this specifically around the formatting of Wikipedia, however it works on many web pages. The program first finds and prints all the titles on the page, and from there prompts the user which they would 
like to learn more about at which point it prints the corresponding paragraphs.<br/>
There a number of issues with the implementation, namely that I did not map the paragraphs to the titles, so for pages with multiple paragraphs per section the numbering system does not align and the paragraphs 
at the end of the document are unreachable. However for this project, I decided to move on rather than fix these problems since I had accomplished my primary goal of parsing a web page.

<h4>Morse Translater</h4>
This program takes in any sentence from the user and translates it into morse code using a HashMap containing all the letters, numbers, and most common characters. Using [Rodio](https://crates.io/crates/rodio), 
the translated sentence is output as audio, and printed to the terminal.

<h4>Sorting Algorithm Comparison</h4>
This program compares the efficiency of six popular sorting algorithms on vectors with sizes ranging from 1 to 1024. The time for each sort is then measured and averaged over 100 trials, before being output to a 
graph containing each of the algorithms. To create the graph, I used [Graplot](https://crates.io/crates/graplot).<br/>
The algorithms compared are merge sort, quick sort, heap sort, shell sort, tim sort, and radix sort. I chose these six as they are some of the fastest and most popular sorting algorithms, and further because I had
never used heap sort or shell sort before. <br/>
My initial goal was to implement all the algorithms myself, however I found this to be too much work for the simple project not to mention my implementations were substantially slower than optimal. Due to this,
I decided to use the algorithms provided on the [Rust Algorithms Github](https://github.com/TheAlgorithms/Rust/tree/master/src/sorting) since they would be faster and more consistent.<br/>
