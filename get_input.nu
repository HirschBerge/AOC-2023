#!/usr/bin/env nu

def --env main [year: string, day: string ] {
  let $cookie_path = $"($env.HOME)/.aochelpers/token"
  let $save_file =  $"($env.HOME)/projects/AdventOfCode/.inputs/($year)/($day)"
  if not ($"(dirname $"($save_file)")" |path exists ) {
    mkdir  -v $"(dirname $"($save_file)")"
  }
  let $url = $"https://adventofcode.com/($year)/day/($day)/input"
  if not ($save_file |path exists) {
    try { curl -s -X GET -H $"Cookie: session=(cat $cookie_path)" $url -o  $save_file} catch {echo "Oopsies"}
  } 
  generate_new_day $day $year
}
def --env generate_new_day [ day: string year: string ] {
  let $formatted_day = $"day_($day)"
  cd $"($env.HOME)/projects/AdventOfCode"
  nix develop -c cargo generate --path ./daily-template/ --name $formatted_day
  cd $"./($formatted_day)/"
  sd '\([\d]{4}, \d\)' $"\(($year), ($day)\)"  src/*.rs
}
