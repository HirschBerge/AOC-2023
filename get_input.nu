#!/usr/bin/env nu

def --env main [year: string, day: string ] {
        let $cookie_path = $"($env.HOME)/.aochelpers/token"
        let $save_file =  $"($env.HOME)/.aochelpers/($year)/($day)"
        let $url = $"https://adventofcode.com/($year)/day/($day)/input"
        echo $"Fetching day: '($day)' from url: '($url)' with token (cat $cookie_path)" | print
        try { wget -X GET -H $"Cookie: session=(cat $cookie_path)" $url -o  $save_file} catch {echo "Oopsies"}
        generate_new_day $day
}
 def --env generate_new_day [ day: string ] {
        let $formatted_day = $"day_($day)"
        cd $"($env.HOME)/projects/AdventOfCode"
        nix develop -c cargo generate --path ./daily-template/ --name $formatted_day
        cd $"./($formatted_day)/"
        sd '\((\d,)' $"\(($day),"  src/lib.rs
 }
